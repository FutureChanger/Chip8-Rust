use rand::Rng;

use crate::cpu::CPU;

//This file contains all the opcodes which a Chip8 provides

pub fn cls(cpu: &mut CPU) {
    for x in 0..64 * 32 {
        cpu.gfx[x] = 0;
    }

    cpu.draw = true;
    cpu.program_counter += 2;
}

pub fn ret(cpu: &mut CPU) {
    cpu.stack_pointer -= 1;
    cpu.program_counter = cpu.stack[cpu.stack_pointer as usize];
}

pub fn jp_addr(cpu: &mut CPU) {
    cpu.program_counter = (cpu.opcode & 0x0FFF) as u16;
}

pub fn call_adr(cpu: &mut CPU) {
    cpu.stack[cpu.stack_pointer as usize] = cpu.program_counter + 2;
    cpu.stack_pointer += 1;
    cpu.program_counter = cpu.opcode & 0x0FFF;
}

pub fn se_vx_byte(cpu: &mut CPU) {
    let value_register_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    if value_register_x == (cpu.opcode & 0x00FF) as u8 {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn sne_vx_byte(cpu: &mut CPU) {
    let value_register_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    if value_register_x != (cpu.opcode & 0x00FF) as u8 {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn se_vx_vy(cpu: &mut CPU) {
    let reg_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let reg_y = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    if reg_x == reg_y {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn ld_vx_byte(cpu: &mut CPU) {
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = (cpu.opcode & 0x00FF) as u8;
    cpu.program_counter += 2;
}

pub fn add_vx_byte(cpu: &mut CPU) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] as u16;
    let sum = vx + (cpu.opcode & 0x00FF) as u16;

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = sum as u8;
    cpu.program_counter += 2;
}

pub fn ld_vx_vy(cpu: &mut CPU) {
    let reg_y = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];
    let reg_x = ((cpu.opcode & 0x0F00) >> 8) as u8;

    cpu.registers[reg_x as usize] = reg_y;
    cpu.program_counter += 2;
}

pub fn or_vx_vy(cpu: &mut CPU) {
    let value_from_reg_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let value_from_reg_y = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = value_from_reg_x | value_from_reg_y;
    cpu.program_counter += 2;
}

pub fn and_vx_vy(cpu: &mut CPU) {
    let value_from_reg_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let value_from_reg_y = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = value_from_reg_x & value_from_reg_y;
    cpu.program_counter += 2;
}

pub fn xor_vx_vy(cpu: &mut CPU) {
    let value_from_reg_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let value_from_reg_y = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = value_from_reg_x ^ value_from_reg_y;
    cpu.program_counter += 2;
}

pub fn add_vx_vy(cpu: &mut CPU) {
    let value_from_reg_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] as u16;
    let value_from_reg_y = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize] as u16;
    let res = (value_from_reg_x + value_from_reg_y) as u16;

    if res > 0xFF {
        cpu.registers[0xF] = 1;
    } else {
        cpu.registers[0xF] = 0;
    }
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = res as u8;
    cpu.program_counter += 2;
}

pub fn sub_vx_vy(cpu: &mut CPU) {
    let value_from_reg_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] as u16;
    let value_from_reg_y = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize] as u16;

    if value_from_reg_x > value_from_reg_y {
        cpu.registers[0xF] = 1;
    } else {
        cpu.registers[0xF] = 0;
    }
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] =
        value_from_reg_x.wrapping_sub(value_from_reg_y) as u8;
    cpu.program_counter += 2;
}

pub fn shr_vx_vy(cpu: &mut CPU) {
    let value_from_reg_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let value_shifted = value_from_reg_x >> 1;
    if value_from_reg_x & 0b00000001 == 0b00000001 {
        cpu.registers[0xF] = 1;
    } else {
        cpu.registers[0xF] = 0;
    }
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = value_shifted;
    cpu.program_counter += 2;
}

pub fn subn_vx_vy(cpu: &mut CPU) {
    let value_from_reg_x = cpu.registers[(((cpu.opcode) & 0x0F00) as usize) >> 8];
    let value_from_reg_y = cpu.registers[(((cpu.opcode) & 0x00F0) as usize) >> 4];

    if value_from_reg_y > value_from_reg_x {
        cpu.registers[0xF] = 1;
    } else {
        cpu.registers[0xF] = 0;
    }

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] =
        value_from_reg_y.wrapping_sub(value_from_reg_x) as u8;
    cpu.program_counter += 2;
}

pub fn shl_vx_vy(cpu: &mut CPU) {
    let value_from_reg_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let value_shifted = value_from_reg_x << 1;
    if value_from_reg_x & 0b10000000 == 0b10000000 {
        cpu.registers[0xF] = 1;
    } else {
        cpu.registers[0xF] = 0;
    }
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = value_shifted;
    cpu.program_counter += 2;
}

pub fn sne_vx_vy(cpu: &mut CPU) {
    let value_from_reg_x = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let value_from_reg_y = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];

    if value_from_reg_x != value_from_reg_y {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn annn_ld_i_addr(cpu: &mut CPU) {
    cpu.index_register = cpu.opcode & 0x0FFF;
    cpu.program_counter += 2;
}

pub fn bnnn_jp_v0_addr(cpu: &mut CPU) {
    cpu.program_counter = (cpu.opcode & 0x0FFF) + (cpu.registers[0x0]) as u16;
    cpu.program_counter += 2;
}

pub fn cxkk_rnd_vx_byte(cpu: &mut CPU) {
    let mask = cpu.opcode & 0x00FF;
    let random_var: u8 = rand::thread_rng().gen();

    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = random_var & mask as u8;
    cpu.program_counter += 2;
}

pub fn dxyn_drw_vx_vy_nibble(cpu: &mut CPU) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    let vy = cpu.registers[((cpu.opcode & 0x00F0) >> 4) as usize];
    let rows = cpu.opcode & 0x000F;

    cpu.registers[0xF] = 0;
    for y in 0..rows {
        let pixel = cpu.memory[(cpu.index_register + y) as usize];

        for x in 0..8 {
            if (pixel & (0x80 >> x)) != 0 {
                let current_position =
                    ((vx as u16 + x as u16) + ((vy as u16 + y) * 64)) % (32 * 64);

                if cpu.gfx[current_position as usize] == 1 {
                    cpu.registers[0xF] = 1;
                }
                cpu.gfx[current_position as usize] ^= 1;
            }
        }
    }

    cpu.draw = true;
    cpu.program_counter += 2;
}

pub fn ex9e_skp_vx(cpu: &mut CPU) {
    let key = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    if cpu.key_state[key as usize] == true {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn exa1_sknp_vx(cpu: &mut CPU) {
    let key = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    if cpu.key_state[key as usize] == false {
        cpu.program_counter += 4;
    } else {
        cpu.program_counter += 2;
    }
}

pub fn fx07_ld_vx_dt(cpu: &mut CPU) {
    cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = cpu.delay_timer;
    cpu.program_counter += 2;
}

pub fn fx0a_ld_vx_k(cpu: &mut CPU) {
    for i in 0..16 {
        if cpu.key_state[i] == true {
            cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] = i as u8;
            cpu.program_counter += 2;
            break;
        }
    }
}

pub fn fx15_ld_dt_vx(cpu: &mut CPU) {
    cpu.delay_timer = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    cpu.program_counter += 2;
}

pub fn fx18_ld_st_vx(cpu: &mut CPU) {
    cpu.sound_timer = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];
    cpu.program_counter += 2;
}

pub fn fx1e_add_i_vx(cpu: &mut CPU) {
    let value = (cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize] as u16) + cpu.index_register;
    cpu.index_register = value;
    cpu.program_counter += 2;
}

pub fn fx29_ld_f_vx(cpu: &mut CPU) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    cpu.index_register = (vx * 0x5) as u16;
    cpu.program_counter += 2;
}

pub fn fx33_ld_b_vx(cpu: &mut CPU) {
    let vx = cpu.registers[((cpu.opcode & 0x0F00) >> 8) as usize];

    cpu.memory[cpu.index_register as usize] = vx / 100;
    cpu.memory[(cpu.index_register + 1) as usize] = (vx / 10) % 10;
    cpu.memory[(cpu.index_register + 2) as usize] = (vx % 100) % 10;
    cpu.program_counter += 2;
}

pub fn fx55_ld_i_vx(cpu: &mut CPU) {
    let x = (cpu.opcode & 0x0F00) >> 8;

    for i in 0..x + 1 {
        cpu.memory[(cpu.index_register + i) as usize] = cpu.registers[i as usize];
    }
    cpu.program_counter += 2;
}

pub fn fx65_ld_vx_i(cpu: &mut CPU) {
    let x = (cpu.opcode & 0x0F00) >> 8;

    for i in 0..x + 1 {
        cpu.registers[i as usize] = cpu.memory[(cpu.index_register + i) as usize];
    }
    cpu.program_counter += 2;
}
use crate::cpu::Cpu;
use nanorand::{Rng, WyRand};

//Opcode: 0NNN
pub fn call(_cpu: &mut Cpu, _address: u16) {
    todo!()
}

//Opcode: 00E0
pub fn clear_screen(cpu: &mut Cpu) {
    cpu.screen.fill(0);
}

//Opcode: 00EE
pub fn return_from_subroutine(cpu: &mut Cpu) {
    cpu.program_counter = cpu.stack[(cpu.stack_pointer - 1) as usize];
    cpu.stack_pointer -= 1;
}

//Opcode: 1NNN
pub fn goto(cpu: &mut Cpu, address: u16) {
    cpu.program_counter = address;
}

//Opcode: 2NNN
pub fn call_sub(cpu: &mut Cpu, address: u16) {
    cpu.stack[cpu.stack_pointer as usize] = cpu.program_counter;
    cpu.stack_pointer += 1;
    cpu.program_counter = address;
}

//Opcode: 3XNN
pub fn skip_if_register_equals(cpu: &mut Cpu, register: u8, value: u8) {
    if cpu.gp_registers[register as usize] == value {
        cpu.program_counter += 2;
    }
}

//Opcode: 4XNN
pub fn skip_if_register_nequals(cpu: &mut Cpu, register: u8, value: u8) {
    if cpu.gp_registers[register as usize] != value {
        cpu.program_counter += 2;
    }
}

//Opcode: 5XY0
pub fn skip_if_register_equals_register(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    if cpu.gp_registers[register_a as usize] == cpu.gp_registers[register_b as usize] {
        cpu.program_counter += 2;
    }
}

//Opcode: 6XNN
pub fn set_register_to_value(cpu: &mut Cpu, register: u8, value: u8) {
    cpu.gp_registers[register as usize] = value;
}

//Opcode: 7XNN
pub fn add_to_register(cpu: &mut Cpu, register: u8, value: u8) {
    cpu.gp_registers[register as usize] = cpu.gp_registers[register as usize].wrapping_add(value);
}

//Opcode: 8XY0
pub fn set_reigster_as_register(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    cpu.gp_registers[register_a as usize] = cpu.gp_registers[register_b as usize];
}

//Opcode: 8XY1
pub fn set_register_as_bitwise_or_register(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    cpu.gp_registers[register_a as usize] |= cpu.gp_registers[register_b as usize];
}

//Opcode: 8XY2
pub fn set_register_as_bitwise_and_register(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    cpu.gp_registers[register_a as usize] &= cpu.gp_registers[register_b as usize];
}

//Opcode: 8XY3
pub fn set_register_as_bitwise_xor_register(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    cpu.gp_registers[register_a as usize] ^= cpu.gp_registers[register_b as usize];
}

//Opcode: 8XY4
pub fn set_register_as_addition(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    let vx = cpu.gp_registers[register_a as usize];
    let vy = cpu.gp_registers[register_b as usize];
    let (result, carry) = vx.overflowing_add(vy);

    cpu.gp_registers[register_a as usize] = result;
    cpu.gp_registers[0xF] = carry as u8;
}

//Opcode: 8XY5
pub fn set_register_as_subtraction(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    let vx = cpu.gp_registers[register_a as usize];
    let vy = cpu.gp_registers[register_b as usize];

    // Subtract Vy from Vx, wrapping around if the result is negative.
    cpu.gp_registers[register_a as usize] = vx.wrapping_sub(vy);

    // Set VF to 1 if there's no borrow, i.e., if Vx > Vy.
    cpu.gp_registers[0xF] = (vx >= vy) as u8;
}

//Opcode: 8XY6
pub fn set_register_as_shift_right(cpu: &mut Cpu, register: u8) {
    // Store least significant bit of register in VF
    let lsb = cpu.gp_registers[register as usize] & 0x1;

    //Shift register to the right by one.
    cpu.gp_registers[register as usize] >>= 1;
    cpu.gp_registers[0xF] = lsb;
}

//Opcode: 8XY7
pub fn set_register_as_b_sub_a(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    let vx = cpu.gp_registers[register_a as usize];
    let vy = cpu.gp_registers[register_b as usize];

    // Subtract Vy from Vx, wrapping around if the result is negative.
    cpu.gp_registers[register_a as usize] = vy.wrapping_sub(vx);

    // Set VF to 1 if there's no borrow, i.e., if Vx > Vy.
    cpu.gp_registers[0xF] = (vy >= vx) as u8;
}

//Opcode: 8XYE
pub fn set_register_as_shift_left(cpu: &mut Cpu, register: u8) {
    //Store most significant bit of register in VF
    let msb = (cpu.gp_registers[register as usize] >> 7) & 0x1;

    //Shift register to the right by one.
    cpu.gp_registers[register as usize] <<= 1;
    cpu.gp_registers[0xF] = msb;
}

//Opcode: 9XY0
pub fn skip_if_register_nequal_register(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    if cpu.gp_registers[register_a as usize] != cpu.gp_registers[register_b as usize] {
        cpu.program_counter += 2;
    }
}

//Opcode: ANNN
pub fn set_i_to_addr(cpu: &mut Cpu, address: u16) {
    cpu.i_register = address;
}

//Opcode: BNNN
pub fn set_i_to_addr_plus_v0(cpu: &mut Cpu, address: u16) {
    cpu.i_register = address + cpu.gp_registers[0] as u16;
}

//Opcode: CXNN
pub fn set_register_as_bitwise_and_with_random(cpu: &mut Cpu, register: u8, value: u8) {
    let mut rng = WyRand::new();
    let random_number = rng.generate::<u8>();

    cpu.gp_registers[register as usize] = value & random_number;
}

//Opcode: DXYN
pub fn draw(cpu: &mut Cpu, register_a: u8, register_b: u8, sprite_height: u8) {
    let x = cpu.gp_registers[register_a as usize] as usize;
    let y = cpu.gp_registers[register_b as usize] as usize;
    let height = sprite_height as usize;
    let starting_location = cpu.i_register;

    for offset in 0..height {
        let mut row = cpu.memory[(starting_location + offset as u16) as usize];

        for column in 0..8 {
            //Get only the first bit and put in in the display
            let index = (x + column) + ((y + offset) * 64);

            if index >= 2047 {
                return;
            }

            let previous_pixel = cpu.screen[index];

            let new_pixel = ((row & 0x80) >> 7) ^ previous_pixel;

            if previous_pixel == 1 && new_pixel == 0 {
                cpu.gp_registers[0xF] = 1;
            }
            row <<= 1;
            cpu.screen[(x + column) + ((y + offset) * 64)] = new_pixel;
        }
    }
}

//Opcode: EX9E
pub fn skip_if_key_pressed(cpu: &mut Cpu, register: u8) {
    let key = cpu.gp_registers[register as usize];
    if cpu.keyboard[key as usize] {
        cpu.program_counter += 2;
    }
}

//Opcode: EXA1
pub fn skip_if_key_npressed(cpu: &mut Cpu, register: u8) {
    let key = cpu.gp_registers[register as usize];
    if !cpu.keyboard[key as usize] {
        cpu.program_counter += 2;
    }
}

//Opcode: FX07
pub fn set_register_to_tregister(cpu: &mut Cpu, register: u8) {
    cpu.gp_registers[register as usize] = cpu.t_register;
}

//Opcode: FX0A
pub fn set_register_to_key(cpu: &mut Cpu, register: u8) {
    cpu.awaited_keypress = Some(register);
}

//Opcode: FX15
pub fn set_tregister_to_register(cpu: &mut Cpu, register: u8) {
    cpu.t_register = cpu.gp_registers[register as usize];
}

//Opcode: FX18
pub fn set_sregister_to_register(cpu: &mut Cpu, register: u8) {
    cpu.s_register = cpu.gp_registers[register as usize];
}

//Opcode: FX1E
pub fn add_register_to_iregister(cpu: &mut Cpu, register: u8) {
    cpu.i_register += cpu.gp_registers[register as usize] as u16;
}

//Opcode: FX29
pub fn set_iregister_to_sprite(cpu: &mut Cpu, register: u8) {
    let value = cpu.gp_registers[register as usize];
    cpu.i_register = value as u16 * 5;
}

//Opcode: FX33
pub fn register_to_decimal(cpu: &mut Cpu, register: u8) {
    cpu.memory[cpu.i_register as usize] = (cpu.gp_registers[register as usize] / 100) % 10;
    cpu.memory[(cpu.i_register + 1) as usize] = (cpu.gp_registers[register as usize] / 10) % 10;
    cpu.memory[(cpu.i_register + 2) as usize] = cpu.gp_registers[register as usize] % 10;
}

//Opcode: FX55
pub fn register_dump_to_memory(cpu: &mut Cpu, register: u8) {
    for i in 0..=register {
        cpu.memory[(cpu.i_register + i as u16) as usize] = cpu.gp_registers[i as usize];
    }
}

//Opcode: FX65
pub fn register_load_from_memory(cpu: &mut Cpu, register: u8) {
    for i in 0..=register {
        cpu.gp_registers[i as usize] = cpu.memory[(cpu.i_register + i as u16) as usize];
    }
}

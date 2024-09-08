mod instruction_set;
pub mod utils;

use core::panic;

pub struct Cpu {
    gp_registers: [u8; 16], //General purpose registers. V16 or VF should not be used by programs.
    i_register: u16,        //I register: generally used to store memory addresses.
    t_register: u8,         //Time register.
    s_register: u8,         //Sound register.

    program_counter: u16,

    stack_pointer: u8,
    stack: [u16; 16],

    memory: [u8; 4096],
    screen: [u8; 64 * 32],

    keyboard: [bool; 16],

    awaited_keypress: Option<u8>,
}

impl Cpu {
    pub fn new(rom_data: Vec<u8>) -> Cpu {
        let mut memory: [u8; 4096] = [0; 4096];

        //Putting fonts in memory
        memory[..80].copy_from_slice(&utils::SPRITES[..80]);

        for (i, byte) in rom_data.into_iter().enumerate() {
            memory[i + 0x200] = byte;
        }

        memory[0x1FF] = 3;

        Cpu {
            gp_registers: [0; 16],
            i_register: 0,
            t_register: 0,
            s_register: 0,
            program_counter: 0x200,

            stack_pointer: 0x0,
            stack: [0; 16],

            memory,

            screen: [0; 64 * 32],

            keyboard: [false; 16],

            awaited_keypress: None,
        }
    }

    pub fn get_screen(&self) -> &[u8; 64 * 32] {
        &self.screen
    }

    pub fn get_gp_registers(&self) -> &[u8; 16] {
        &self.gp_registers
    }

    pub fn get_i_register(&self) -> u16 {
        self.i_register
    }

    pub fn get_t_register(&self) -> u8 {
        self.t_register
    }

    pub fn get_s_register(&self) -> u8 {
        self.s_register
    }
    pub fn get_stack_pointer(&self) -> u8 {
        self.stack_pointer
    }

    pub fn get_stack(&self) -> &[u16; 16] {
        &self.stack
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn get_current_instruction(&self) -> u16 {
        //Get first part
        let mut instruction: u16 = (self.memory[self.program_counter as usize] as u16) << 8;
        instruction |= self.memory[(self.program_counter + 1) as usize] as u16;
        //Get second part
        // (self.memory[self.program_counter as usize] <<8)
        instruction
    }

    pub fn set_key(&mut self, key: u8) {
        self.keyboard[key as usize] = true;

        if let Some(reg) = self.awaited_keypress {
            self.gp_registers[reg as usize] = key;
            self.awaited_keypress = None;
        }
    }

    //TODO: make this return Result<(), &str>. So by analyzing it from outside we can know if its blocked on input or not.
    pub fn clock(&mut self) {
        self.memory[0x1FF] = 1;

        //If we are in blocking mode just return, change this.
        if self.awaited_keypress.is_some() {
            return;
        }

        //Fetch
        let opcode_1: u8 = self.memory[self.program_counter as usize] >> 4;
        let opcode_2: u8 = (self.memory[self.program_counter as usize] << 4) >> 4;
        let opcode_3: u8 = self.memory[(self.program_counter + 1) as usize] >> 4;
        let opcode_4: u8 = (self.memory[(self.program_counter + 1) as usize] << 4) >> 4;

        self.program_counter += 2;

        //Decode and execute
        match opcode_1 {
            0 => match (opcode_2, opcode_4) {
                (0x1..=0x8, 0x1..=0x8) => instruction_set::call(
                    self,
                    utils::opcode_to_address(opcode_2, opcode_3, opcode_4),
                ),
                (0x0, 0xE) => instruction_set::return_from_subroutine(self),
                (0x0, 0x0) => instruction_set::clear_screen(self),
                _ => panic!("unsupported operation!"),
            },

            0x1 => {
                instruction_set::goto(self, utils::opcode_to_address(opcode_2, opcode_3, opcode_4))
            }
            0x2 => instruction_set::call_sub(
                self,
                utils::opcode_to_address(opcode_2, opcode_3, opcode_4),
            ),
            0x3 => instruction_set::skip_if_register_equals(
                self,
                opcode_2,
                utils::opcode_to_value(opcode_3, opcode_4),
            ),
            0x4 => instruction_set::skip_if_register_nequals(
                self,
                opcode_2,
                utils::opcode_to_value(opcode_3, opcode_4),
            ),
            0x5 => instruction_set::skip_if_register_equals_register(self, opcode_2, opcode_3),
            0x6 => instruction_set::set_register_to_value(
                self,
                opcode_2,
                utils::opcode_to_value(opcode_3, opcode_4),
            ),
            0x7 => instruction_set::add_to_register(
                self,
                opcode_2,
                utils::opcode_to_value(opcode_3, opcode_4),
            ),
            0x8 => match opcode_4 {
                0 => instruction_set::set_reigster_as_register(self, opcode_2, opcode_3),
                1 => instruction_set::set_register_as_bitwise_or_register(self, opcode_2, opcode_3),
                2 => {
                    instruction_set::set_register_as_bitwise_and_register(self, opcode_2, opcode_3)
                }
                3 => {
                    instruction_set::set_register_as_bitwise_xor_register(self, opcode_2, opcode_3)
                }
                4 => instruction_set::set_register_as_addition(self, opcode_2, opcode_3),
                5 => instruction_set::set_register_as_subtraction(self, opcode_2, opcode_3),
                6 => instruction_set::set_register_as_shift_right(self, opcode_2),
                7 => instruction_set::set_register_as_b_sub_a(self, opcode_2, opcode_3),
                14 => instruction_set::set_register_as_shift_left(self, opcode_2),
                _ => panic!("unsupported operation!"),
            },
            0x9 => instruction_set::skip_if_register_nequal_register(self, opcode_2, opcode_3),
            0xA => instruction_set::set_i_to_addr(
                self,
                utils::opcode_to_address(opcode_2, opcode_3, opcode_4),
            ),
            0xB => instruction_set::set_i_to_addr_plus_v0(
                self,
                utils::opcode_to_address(opcode_2, opcode_3, opcode_4),
            ),
            0xC => instruction_set::set_register_as_bitwise_and_with_random(
                self,
                opcode_2,
                utils::opcode_to_value(opcode_3, opcode_4),
            ),
            0xD => instruction_set::draw(self, opcode_2, opcode_3, opcode_4),
            0xE => match (opcode_3, opcode_4) {
                (0x9, 0xE) => instruction_set::skip_if_key_pressed(self, opcode_2),
                (0xA, 0x1) => instruction_set::skip_if_key_npressed(self, opcode_2),
                _ => panic!(
                    "unsupported operation! {:#x}{:#x}{:#x}{:#x}",
                    opcode_1, opcode_2, opcode_3, opcode_4
                ),
            },
            0xF => match (opcode_3, opcode_4) {
                (0x0, 0x7) => instruction_set::set_register_to_tregister(self, opcode_2),
                (0x0, 0xA) => instruction_set::set_register_to_key(self, opcode_2),
                (0x1, 0x5) => instruction_set::set_tregister_to_register(self, opcode_2),
                (0x1, 0x8) => instruction_set::set_sregister_to_register(self, opcode_2),
                (0x1, 0xE) => instruction_set::add_register_to_iregister(self, opcode_2),
                (0x2, 0x9) => instruction_set::set_iregister_to_sprite(self, opcode_2),
                (0x3, 0x3) => instruction_set::register_to_decimal(self, opcode_2),
                (0x5, 0x5) => instruction_set::register_dump_to_memory(self, opcode_2),
                (0x6, 0x5) => instruction_set::register_load_from_memory(self, opcode_2),
                _ => panic!("unsupported operation!"),
            },
            _ => panic!("unsupported operation!"),
        }
    }
}

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;

//TODO: Read from main args.
fn main() {
    //Read file containing ROM
    let rom_data = read_rom("15.ch8").unwrap_or_else(|err| {
        println!("Problem reading rom_file: {err}");
        process::exit(1);
    });

    //Create dump file
    let mut output = File::create("dump.txt").unwrap();

    for i in (0..rom_data.len()).step_by(2) {
        let opcode_1: u8 = rom_data[i] >> 4;
        let opcode_2: u8 = (rom_data[i] << 4) >> 4;
        let opcode_3: u8 = rom_data[i + 1] >> 4;
        let opcode_4: u8 = (rom_data[i + 1] << 4) >> 4;

        //Decode and execute
        match opcode_1 {
            0 => match (opcode_2, opcode_4) {
                (0x1..=0x8, 0x1..=0x8) => output.write_fmt(format_args!(
                    "call {}",
                    opcode_to_address(opcode_2, opcode_3, opcode_4)
                )),
                (0x0, 0xE) => output.write_fmt(format_args!("return")),
                (0x0, 0x0) => output.write_fmt(format_args!("clscr")),
                _ => panic!("unsupported operation!"),
            },

            0x1 => self.goto(Cpu::opcode_to_address(opcode_2, opcode_3, opcode_4)),
            0x2 => self.call_sub(Cpu::opcode_to_address(opcode_2, opcode_3, opcode_4)),
            0x3 => self.skip_if_register_equals(opcode_2, Cpu::opcode_to_value(opcode_3, opcode_4)),
            0x4 => {
                self.skip_if_register_nequals(opcode_2, Cpu::opcode_to_value(opcode_3, opcode_4))
            }
            0x5 => self.skip_if_register_equals_register(opcode_2, opcode_3),
            0x6 => self.set_register_to_value(opcode_2, Cpu::opcode_to_value(opcode_3, opcode_4)),
            0x7 => self.add_to_register(opcode_2, Cpu::opcode_to_value(opcode_3, opcode_4)),
            0x8 => match opcode_4 {
                0 => self.set_reigster_as_register(opcode_2, opcode_3),
                1 => self.set_register_as_bitwise_or_register(opcode_2, opcode_3),
                2 => self.set_register_as_bitwise_and_register(opcode_2, opcode_3),
                3 => self.set_register_as_bitwise_xor_register(opcode_2, opcode_3),
                4 => self.set_register_as_addition(opcode_2, opcode_3),
                5 => self.set_register_as_subtraction(opcode_2, opcode_3),
                6 => self.set_register_as_shift_right(opcode_2),
                7 => self.set_register_as_b_sub_a(opcode_2, opcode_3),
                14 => self.set_register_as_shift_left(opcode_2),
                _ => panic!("unsupported operation!"),
            },
            0x9 => self.skip_if_register_nequal_register(opcode_2, opcode_3),
            0xA => self.set_i_to_addr(Cpu::opcode_to_address(opcode_2, opcode_3, opcode_4)),
            0xB => self.set_i_to_addr_plus_v0(Cpu::opcode_to_address(opcode_2, opcode_3, opcode_4)),
            0xC => self.set_register_as_bitwise_and_with_random(
                opcode_2,
                Cpu::opcode_to_value(opcode_3, opcode_4),
            ),
            0xD => self.draw(opcode_2, opcode_3, opcode_4),
            0xE => match (opcode_3, opcode_4) {
                (0x9, 0xE) => self.skip_if_key_pressed(opcode_2),
                (0xA, 0x1) => self.skip_if_key_npressed(opcode_2),
                _ => panic!(
                    "unsupported operation! {:#x}{:#x}{:#x}{:#x}",
                    opcode_1, opcode_2, opcode_3, opcode_4
                ),
            },
            0xF => match (opcode_3, opcode_4) {
                (0x0, 0x7) => self.set_register_to_tregister(opcode_2),
                (0x0, 0xA) => self.set_register_to_key(opcode_2),
                (0x1, 0x5) => self.set_tregister_to_register(opcode_2),
                (0x1, 0x8) => self.set_sregister_to_register(opcode_2),
                (0x1, 0xE) => self.add_register_to_iregister(opcode_2),
                (0x2, 0x9) => self.set_iregister_to_sprite(opcode_2),
                (0x3, 0x3) => self.register_to_decimal(opcode_2),
                (0x5, 0x5) => self.register_dump_to_memory(opcode_2),
                (0x6, 0x5) => self.register_load_from_memory(opcode_2),
                _ => panic!("unsupported operation!"),
            },
            _ => panic!("unsupported operation!"),
        }
    }
}

fn opcode_to_address(opcode_1: u8, opcode_2: u8, opcode_3: u8) -> u16 {
    let mut address = 0;
    address |= (opcode_1 as u16) << 8;
    address |= (opcode_2 as u16) << 4;
    address |= opcode_3 as u16;
    address
}

fn opcode_to_value(opcode_1: u8, opcode_2: u8) -> u8 {
    let mut value: u8 = 0;
    value |= (opcode_1) << 4;
    value |= opcode_2;
    value
}

fn read_rom(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let rom_data = fs::read(path)?;

    Ok(rom_data)
}

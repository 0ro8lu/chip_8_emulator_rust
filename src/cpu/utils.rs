use std::error::Error;
use std::fs;

pub const SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, /* 0 */
    0x20, 0x60, 0x20, 0x20, 0x70, /* 1 */
    0xF0, 0x10, 0xF0, 0x80, 0xF0, /* 2 */
    0xF0, 0x10, 0xF0, 0x10, 0xF0, /* 3 */
    0x90, 0x90, 0xF0, 0x10, 0x10, /* 4 */
    0xF0, 0x80, 0xF0, 0x10, 0xF0, /* 5 */
    0xF0, 0x80, 0xF0, 0x90, 0xF0, /* 6 */
    0xF0, 0x10, 0x20, 0x40, 0x40, /* 7 */
    0xF0, 0x90, 0xF0, 0x90, 0xF0, /* 8 */
    0xF0, 0x90, 0xF0, 0x10, 0xF0, /* 9 */
    0xF0, 0x90, 0xF0, 0x90, 0x90, /* a */
    0xE0, 0x90, 0xE0, 0x90, 0xE0, /* b */
    0xF0, 0x80, 0x80, 0x80, 0xF0, /* c */
    0xE0, 0x90, 0x90, 0x90, 0xE0, /* d */
    0xF0, 0x80, 0xF0, 0x80, 0xF0, /* e */
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub fn opcode_to_address(opcode_1: u8, opcode_2: u8, opcode_3: u8) -> u16 {
    let mut address = 0;
    address |= (opcode_1 as u16) << 8;
    address |= (opcode_2 as u16) << 4;
    address |= opcode_3 as u16;
    address
}

pub fn opcode_to_value(opcode_1: u8, opcode_2: u8) -> u8 {
    let mut value: u8 = 0;
    value |= (opcode_1) << 4;
    value |= opcode_2;
    value
}

pub fn read_rom(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let rom_data = fs::read(path)?;
    Ok(rom_data)
}

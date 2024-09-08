use std::process;

use super::EmulatorClient;
use crate::cpu::utils;
use crate::cpu::Cpu;

pub struct DebugClient {
    cpu: Cpu,
}

impl EmulatorClient for DebugClient {
    fn build(mut rom_path: Option<&String>) -> Self {
        let rom_data = if let Some(path) = rom_path.take() {
            utils::read_rom(path.as_str()).unwrap_or_else(|err| {
                eprintln!("Problem reading rom_file: {err}");
                process::exit(1);
            })
        } else {
            eprintln!("Please specify the path to a valid rom in the command!");
            process::exit(1);
        };

        DebugClient {
            cpu: Cpu::new(rom_data),
        }
    }
    fn update(&mut self) {
        loop {
            self.cpu.clock();
        }
    }
}

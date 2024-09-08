use std::env;

mod cpu;
use crate::cpu::Cpu;

mod client;
use client::DebugClient;
use client::EmulatorClient;
use client::NcursesClient;

fn main() {
    //Read arguments
    let args: Vec<String> = env::args().collect();
    //Create the emulator client
    let mut emulator_client = NcursesClient::build(args.get(1));
    // let mut emulator_client = DebugClient::build(args.get(1));
    emulator_client.update();
}

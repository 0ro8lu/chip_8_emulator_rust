// use std::error::Error;
// use std::fs;

mod debug_client;
mod ncurses_client;

pub use debug_client::DebugClient;
pub use ncurses_client::NcursesClient;

pub trait EmulatorClient {
    fn build(rom_path: Option<&String>) -> Self;
    fn update(&mut self);
    //TODO: have a reset method for resetting the emulator.
}

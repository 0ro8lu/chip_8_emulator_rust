use std::process;

extern crate ncurses;

use super::EmulatorClient;
use crate::cpu::utils;
use crate::Cpu;

use ncurses::*;
use std::{thread, time};

pub struct NcursesClient {
    cpu: Cpu,
    emulator_window: WINDOW,
    debug_window: WINDOW,
    paused: bool,
    debug: bool,
    single_step: bool,
}

impl NcursesClient {
    fn shutdown(&mut self) {
        endwin();
    }

    fn render(&self) {
        let screen = self.cpu.get_screen();

        for y in 0..32 {
            for x in 0..64 {
                let pixel;
                if screen[x + (y * 64)] == 1 {
                    pixel = char::from_u32(0x2588).unwrap();
                } else {
                    pixel = ' ';
                }

                mvwaddstr(
                    self.emulator_window,
                    //plus one otherwise we will draw on top of the window borders
                    //thanks ncurses for being like that
                    (y + 1) as i32,
                    (x + 1) as i32,
                    format!("{}", pixel).as_ref(),
                );
            }
        }

        //-------------- Debug info --------------

        if self.debug {
            //Print gp_register contents
            let registers = self.cpu.get_gp_registers();
            for i in 0..16 {
                mvwaddstr(
                    self.debug_window,
                    1 + i,
                    1,
                    format!("V{i}: 0x{:x}", registers[i as usize]).as_ref(),
                );
            }

            //Print special registers
            mvwaddstr(
                self.debug_window,
                18,
                1,
                format!("i_reg: 0x{:x}", self.cpu.get_i_register()).as_ref(),
            );

            mvwaddstr(
                self.debug_window,
                19,
                1,
                format!("t_reg: 0x{:x}", self.cpu.get_t_register()).as_ref(),
            );

            mvwaddstr(
                self.debug_window,
                20,
                1,
                format!("i_reg: 0x{:x}", self.cpu.get_s_register()).as_ref(),
            );
            //Print stack and stack pointer
            let stack_pointer = self.cpu.get_stack_pointer();
            let stack = self.cpu.get_stack();
            for i in 0..16 {
                mvwaddstr(
                    self.debug_window,
                    1 + i,
                    17,
                    format!("Stack {i}: 0x{:x}", stack[i as usize]).as_ref(),
                );
            }
            mvwaddstr(
                self.debug_window,
                18,
                17,
                format!("Stack Pointer: 0x{:x}", stack_pointer).as_ref(),
            );

            //Print program counter
            let program_counter = self.cpu.get_program_counter();
            mvwaddstr(
                self.debug_window,
                2,
                35,
                format!("program counter: 0x{:x}", program_counter).as_ref(),
            );

            //Print current opcode
            let instruction = self.cpu.get_current_instruction();
            mvwaddstr(
                self.debug_window,
                1,
                35,
                format!("instruction: 0x{:x}", instruction).as_ref(),
            );
            wrefresh(self.debug_window);
        }

        wrefresh(self.emulator_window);
        refresh();
    }

    fn input_action(&mut self, input: i32) {
        //Decode user input
        match input as u8 {
            b'q' => {
                self.shutdown();
                process::exit(0);
            } // Quit
            b' ' => self.paused = !self.paused, // Pause/Un-pause
            b'd' => {
                // Debug view or not
                if self.debug {
                    wclear(self.debug_window);
                } else {
                    box_(self.debug_window, 0, 0);
                }

                wrefresh(self.debug_window);
                self.debug = !self.debug;
            }
            b's' => {
                if self.single_step {
                    //Non-blocking input
                    timeout(10);
                } else {
                    //Blocking input
                    timeout(-10);
                }

                self.single_step = !self.single_step;
            }
            _ => (),
        }
    }
}

impl EmulatorClient for NcursesClient {
    fn build(mut rom_path: Option<&String>) -> Self {
        //TOOD: If path is Some, use it. Otherwise ask the user.
        let rom_data = if let Some(path) = rom_path.take() {
            utils::read_rom(path.as_str()).unwrap_or_else(|err| {
                eprintln!("Problem reading rom_file: {err}");
                process::exit(1);
            })
        } else {
            eprintln!("Please specify the path to a valid rom in the command!");
            process::exit(1);
        };

        //ncurses initialization

        //Add support for UTF-8
        let locale_conf = LcCategory::all;
        setlocale(locale_conf, "en_US.UTF-8");

        //run of the mill initialization + non-blocking getch
        initscr();
        noecho();
        timeout(10);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        //Making layout for emulator
        //Will have 3 windows:
        //1) emulator output screen
        //2) debug register screen
        //3) debug memory output

        //Emulator window (note: had to do dimension + 2 as otherwise border)
        //is considered as a pixel
        let emulator_window = newwin(34, 66, 0, 0);
        box_(emulator_window, 0, 0);
        refresh();
        wrefresh(emulator_window);

        //Debug Window
        let debug_window = newwin(34, 66, 0, 67);
        box_(debug_window, 0, 0);

        NcursesClient {
            cpu: Cpu::new(rom_data),
            emulator_window,
            debug_window,
            paused: true,
            debug: false,
            single_step: false,
        }
    }

    fn update(&mut self) {
        loop {
            let ten_millis = time::Duration::from_millis(2);
            thread::sleep(ten_millis);

            //Read user input
            let input = getch();
            self.input_action(input);

            //Only update emulator and render if not paused

            self.render();

            if (self.single_step && !self.paused && input as u8 == b'n')
                || (!self.single_step && !self.paused)
            {
                self.cpu.clock();
            }
        }
    }
}

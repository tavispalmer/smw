use std::ffi::{c_void, CStr};

use gfx::Gfx;

use crate::{cpu::Cpu, mem::Mem, rom::Rom};

mod cpu;
mod ffi;
mod mem;
mod retro;
mod rom;

pub struct App {
    gfx: Option<Gfx>,
    cpu: Cpu,
}

impl App {
    pub fn load_game(data: &[u8]) -> Self {
        Self {
            gfx: None,
            cpu: Cpu::new(Mem::new(Rom::new(data))),
        }
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }

    pub fn context_reset<F: FnMut(&CStr) -> *const c_void>(&mut self, f: F) {
        self.gfx = Some(Gfx::new(f));
    }

    pub fn context_destroy(&mut self) {
        self.gfx = None;
    }
}

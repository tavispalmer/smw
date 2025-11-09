use std::ffi::{c_void, CStr};

use gfx::Gfx;

use crate::rom::Rom;

mod cpu;
mod ffi;
mod mem;
mod retro;
mod rom;

pub struct App {
    gfx: Option<Gfx>,
    rom: Rom,
}

impl App {
    pub fn load_game(data: &[u8]) -> Self {
        Self {
            gfx: None,
            rom: Rom::new(data),
        }
    }

    pub fn context_reset<F: FnMut(&CStr) -> *const c_void>(&mut self, f: F) {
        self.gfx = Some(Gfx::new(f));
    }

    pub fn context_destroy(&mut self) {
        self.gfx = None;
    }
}

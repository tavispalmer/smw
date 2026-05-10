use std::{
    ffi::{c_uint, c_void},
    slice,
};

use crate::sys::{SfcCallbacks, sfc_init, sfc_iter, sfc_quit};

mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub trait Callbacks {
    fn video_refresh(&mut self, data: &[u32], width: usize, height: usize, pitch: usize);
    fn audio_sample_batch(&mut self, data: &[i16]) -> usize;
    fn input_poll(&mut self);
    fn input_state(&mut self, port: u32, device: u32, index: u32, id: u32) -> i16;
}

pub struct Smw<C: Callbacks> {
    pub c: C,
}

impl<C: Callbacks> Smw<C> {
    pub fn new(mut c: C, rom: &[u8]) -> Self {
        const CLBK: SfcCallbacks = SfcCallbacks {
            video_refresh: Some({
                extern "C" fn video_refresh(
                    user: *mut c_void,
                    data: *const c_void,
                    width: c_uint,
                    height: c_uint,
                    pitch: usize,
                ) {
                    let this = unsafe { &mut *user.cast::<&mut dyn Callbacks>() };
                    this.video_refresh(
                        unsafe {
                            slice::from_raw_parts(
                                data.cast(),
                                height as usize * pitch / size_of::<u32>(),
                            )
                        },
                        width as usize,
                        height as usize,
                        pitch,
                    )
                }
                video_refresh
            }),
            audio_sample_batch: Some({
                extern "C" fn audio_sample_batch(
                    user: *mut c_void,
                    data: *const i16,
                    frames: usize,
                ) -> usize {
                    let this = unsafe { &mut *user.cast::<&mut dyn Callbacks>() };
                    this.audio_sample_batch(unsafe { slice::from_raw_parts(data, frames * 2) })
                }
                audio_sample_batch
            }),
            input_poll: Some({
                extern "C" fn input_poll(user: *mut c_void) {
                    let this = unsafe { &mut *user.cast::<&mut dyn Callbacks>() };
                    this.input_poll()
                }
                input_poll
            }),
            input_state: Some({
                extern "C" fn input_state(
                    user: *mut c_void,
                    port: c_uint,
                    device: c_uint,
                    index: c_uint,
                    id: c_uint,
                ) -> i16 {
                    let this = unsafe { &mut *user.cast::<&mut dyn Callbacks>() };
                    this.input_state(port, device, index, id)
                }
                input_state
            }),
        };

        {
            let mut callbacks: &mut dyn Callbacks = &mut c;
            unsafe {
                sfc_init((&raw mut callbacks).cast(), &CLBK, rom.as_ptr(), rom.len());
            }
        }

        Self { c }
    }

    pub fn iter(&mut self) {
        let mut callbacks: &mut dyn Callbacks = &mut self.c;
        unsafe {
            sfc_iter((&raw mut callbacks).cast());
        }
    }
}

impl<C: Callbacks> Drop for Smw<C> {
    fn drop(&mut self) {
        let mut callbacks: &mut dyn Callbacks = &mut self.c;
        unsafe {
            sfc_quit((&raw mut callbacks).cast());
        }
    }
}

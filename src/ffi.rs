// libretro exports

use std::{ffi::{c_char, c_uint, c_void}, mem::{self, MaybeUninit}, ptr, slice};

use crate::{retro, App};

static mut APP: MaybeUninit<App> = MaybeUninit::uninit();

static mut ENVIRON_CB: Option<retro::environment_t> = None;
static mut VIDEO_CB: Option<retro::video_refresh_t> = None;
static mut AUDIO_CB: Option<retro::audio_sample_t> = None;
static mut AUDIO_BATCH_CB: Option<retro::audio_sample_batch_t> = None;
static mut INPUT_POLL_CB: Option<retro::input_poll_t> = None;
static mut INPUT_STATE_CB: Option<retro::input_state_t> = None;

static mut HW_RENDER: retro::hw_render_callback = retro::hw_render_callback {
    context_type: retro::HW_CONTEXT_OPENGL_CORE,
    context_reset: Some(context_reset),
    get_current_framebuffer: None,
    get_proc_address: None,
    depth: true,
    stencil: true,
    bottom_left_origin: true,
    version_major: 3,
    version_minor: 1,
    cache_context: false,
    context_destroy: Some(context_destroy),
    debug_context: false,
};

extern "C" fn context_reset() {
    eprintln!("Context reset!");

    unsafe {
        let get_proc_address = HW_RENDER.get_proc_address.unwrap_unchecked();
        #[allow(static_mut_refs)]
        APP.assume_init_mut().context_reset(|sym| mem::transmute(get_proc_address(sym.as_ptr())));
    }
}

extern "C" fn context_destroy() {
    eprintln!("Context destroy!");

    unsafe {
        #[allow(static_mut_refs)]
        APP.assume_init_mut().context_destroy();
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_set_environment(cb: retro::environment_t) {
    unsafe {
        ENVIRON_CB = Some(cb);
        // let environ_cb = ENVIRON_CB.unwrap_unchecked();

        // let no_rom = true;
        // environ_cb(retro::ENVIRONMENT_SET_SUPPORT_NO_GAME, &raw const no_rom as _);
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_set_video_refresh(cb: retro::video_refresh_t) {
    unsafe {
        VIDEO_CB = Some(cb)
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_set_audio_sample(cb: retro::audio_sample_t) {
    unsafe {
        AUDIO_CB = Some(cb)
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_set_audio_sample_batch(cb: retro::audio_sample_batch_t) {
    unsafe {
        AUDIO_BATCH_CB = Some(cb)
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_set_input_poll(cb: retro::input_poll_t) {
    unsafe {
        INPUT_POLL_CB = Some(cb)
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_set_input_state(cb: retro::input_state_t) {
    unsafe {
        INPUT_STATE_CB = Some(cb)
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_init() {}

#[unsafe(no_mangle)]
extern "C" fn retro_deinit() {}

#[unsafe(no_mangle)]
extern "C" fn retro_api_version() -> c_uint { retro::API_VERSION }

#[unsafe(no_mangle)]
extern "C" fn retro_get_system_info(info: *mut retro::system_info) {
    unsafe {
        *info = retro::system_info::default()
            .library_name(c"smw")
            .library_version(c"v1");
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_get_system_av_info(info: *mut retro::system_av_info) {
    unsafe {
        *info = retro::system_av_info::default()
            .geometry(retro::game_geometry::default()
                .base_width(256)
                .base_height(240)
                .max_width(256)
                .max_height(240)
                .aspect_ratio(4.0 / 3.0))
            .timing(retro::system_timing::default()
                .fps(60.0));
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_set_controller_port_device(_port: c_uint, _device: c_uint) {}

#[unsafe(no_mangle)]
extern "C" fn retro_reset() {}

#[unsafe(no_mangle)]
extern "C" fn retro_run() {}

#[unsafe(no_mangle)]
extern "C" fn retro_serialize_size() -> usize { 0 }

#[unsafe(no_mangle)]
extern "C" fn retro_serialize(_data: *mut c_void, _len: usize) -> bool { false }

#[unsafe(no_mangle)]
extern "C" fn retro_unserialize(_data: *const c_void, _len: usize) -> bool { false }

#[unsafe(no_mangle)]
extern "C" fn retro_cheat_reset() {}

#[unsafe(no_mangle)]
extern "C" fn retro_cheat_set(_index: c_uint, _enabled: bool, _code: *const c_char) {}

#[unsafe(no_mangle)]
extern "C" fn retro_load_game(game: *const retro::game_info) -> bool {
    unsafe {
        let environ_cb = ENVIRON_CB.unwrap_unchecked();

        let fmt = retro::PIXEL_FORMAT_XRGB8888;
        if !environ_cb(retro::ENVIRONMENT_SET_PIXEL_FORMAT, &raw const fmt as _) {
            eprintln!("XRGB8888 is not supported.");
            return false;
        }

        if !environ_cb(retro::ENVIRONMENT_SET_HW_RENDER, &raw mut HW_RENDER as _) {
            return false;
        }

        let game = &*game;

        #[allow(static_mut_refs)]
        APP.write(App::load_game(
            slice::from_raw_parts(game.data as _, game.size),
        ));

        eprintln!("Loaded game!");
        true
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_load_game_special(_game_type: c_uint, _info: *const retro::game_info, _num_info: usize) -> bool { false }

#[unsafe(no_mangle)]
extern "C" fn retro_unload_game() {
    unsafe {
        #[allow(static_mut_refs)]
        APP.assume_init_drop();
    }
}

#[unsafe(no_mangle)]
extern "C" fn retro_get_region() -> c_uint { retro::REGION_NTSC }

#[unsafe(no_mangle)]
extern "C" fn retro_get_memory_data(_id: c_uint) -> *mut c_void { ptr::null_mut() }

#[unsafe(no_mangle)]
extern "C" fn retro_get_memory_size(_id: c_uint) -> usize { 0 }

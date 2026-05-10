use std::{
    ffi::{CStr, c_char, c_int, c_void},
    fs,
    process::ExitCode,
    ptr,
};

use sdl3_sys::{
    audio::*, error::*, events::*, gamepad::*, init::*, log::*, main::*, pixels::*, surface::*,
    video::*,
};
use smw::{Callbacks, Smw};

struct AppState {
    pub window: *mut SDL_Window,
    pub surface: *mut SDL_Surface,
    pub stream: *mut SDL_AudioStream,
    pub gamepad: *mut SDL_Gamepad,
    pub gamepad_state: u16,
}

impl Callbacks for AppState {
    fn video_refresh(&mut self, data: &[u32], width: usize, height: usize, pitch: usize) {
        let surface = unsafe { &mut *self.surface };

        unsafe {
            if SDL_MUSTLOCK(surface) {
                SDL_LockSurface(surface);
            }
            for y in 0..224 as usize {
                for x in 46..256 + 46 as usize {
                    *surface
                        .pixels
                        .cast::<u32>()
                        .add(y * surface.pitch as usize / size_of::<u32>() + x) =
                        data[y * pitch / size_of::<u32>() + x - 46];
                }
            }
            if SDL_MUSTLOCK(surface) {
                SDL_UnlockSurface(surface);
            }

            let window_surface = &mut *SDL_GetWindowSurface(self.window);
            SDL_BlitSurfaceScaled(
                surface,
                ptr::null(),
                window_surface,
                ptr::null(),
                SDL_SCALEMODE_NEAREST,
            );
            SDL_UpdateWindowSurface(self.window);
        }
    }

    fn audio_sample_batch(&mut self, data: &[i16]) -> usize {
        unsafe {
            SDL_PutAudioStreamData(
                self.stream,
                data.as_ptr().cast(),
                (data.len() * size_of::<i16>()) as i32,
            );
        }
        data.len() / 2
    }

    fn input_poll(&mut self) {
        // poll the buttons we care about
        if !self.gamepad.is_null() {
            unsafe {
                self.gamepad_state = ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_SOUTH)
                    as u16)
                    << 0)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_WEST) as u16) << 1)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_BACK) as u16) << 2)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_START) as u16) << 3)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_DPAD_UP) as u16)
                        << 4)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_DPAD_DOWN) as u16)
                        << 5)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_DPAD_LEFT) as u16)
                        << 6)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_DPAD_RIGHT) as u16)
                        << 7)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_EAST) as u16) << 8)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_NORTH) as u16) << 9)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_LEFT_SHOULDER)
                        as u16)
                        << 10)
                    | ((SDL_GetGamepadButton(self.gamepad, SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER)
                        as u16)
                        << 11);
            }
        } else {
            self.gamepad_state = 0;
        }
    }

    fn input_state(&mut self, port: u32, device: u32, index: u32, id: u32) -> i16 {
        if port == 0 && device == 1 && index == 0 {
            ((self.gamepad_state & (1 << id)) != 0) as i16
        } else {
            0
        }
    }
}

extern "C" fn appinit(
    appstate: *mut *mut c_void,
    _argc: c_int,
    _argv: *mut *mut c_char,
) -> SDL_AppResult {
    unsafe {
        if !SDL_SetAppMetadata(
            c"smw".as_ptr(),
            c"1.0".as_ptr(),
            c"com.tavispalmer.smw".as_ptr(),
        ) {
            return SDL_APP_FAILURE;
        }

        if !SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMEPAD) {
            SDL_Log(c"Couldn't initialize SDL: %s".as_ptr(), SDL_GetError());
            return SDL_APP_FAILURE;
        }

        let window = SDL_CreateWindow(
            c"smw".as_ptr(),
            398 * 3,
            // (256.0f64 * ((135000000.0 / 11.0) / (21477272.0 / 2.0))).round() as i32 * 3, // 293
            224 * 3,
            SDL_WindowFlags(0),
        );
        if window.is_null() {
            return SDL_APP_FAILURE;
        }

        let surface = SDL_CreateSurface(348, 224, SDL_PIXELFORMAT_XRGB8888);

        let spec = SDL_AudioSpec {
            format: SDL_AUDIO_S16,
            channels: 2,
            freq: 32040.5 as i32,
        };

        let stream = SDL_OpenAudioDeviceStream(
            SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK,
            &spec,
            None,
            ptr::null_mut(),
        );
        if stream.is_null() {
            SDL_Log(c"Couldn't create audio stream: %s".as_ptr(), SDL_GetError());
            return SDL_APP_FAILURE;
        }

        SDL_ResumeAudioStreamDevice(stream);

        // SDL_SetRenderLogicalPresentation(renderer, 800, 600, SDL_LOGICAL_PRESENTATION_LETTERBOX);

        // load game
        let game = fs::read("smw.sfc").unwrap();

        *appstate = Box::into_raw(Box::new(Smw::new(
            AppState {
                window,
                surface,
                stream,
                gamepad: ptr::null_mut(),
                gamepad_state: 0,
            },
            &game,
        )))
        .cast();

        SDL_APP_CONTINUE
    }
}

extern "C" fn appiter(appstate: *mut c_void) -> SDL_AppResult {
    let smw = unsafe { &mut *appstate.cast::<Smw<AppState>>() };
    let appstate = &mut smw.c;

    // do we need more audio?
    if unsafe { SDL_GetAudioStreamQueued(appstate.stream) < 0x1000 } {
        // run the game loop to get more audio
        // this synchronizes audio and video for us :)
        smw.iter();
    }

    SDL_APP_CONTINUE
}

extern "C" fn appevent(appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
    let smw = unsafe { &mut *appstate.cast::<Smw<AppState>>() };
    let appstate = &mut smw.c;
    let event = unsafe { &mut *event };
    match SDL_EventType(unsafe { event.r#type }) {
        SDL_EVENT_QUIT => return SDL_APP_SUCCESS,
        SDL_EVENT_GAMEPAD_ADDED => unsafe {
            if appstate.gamepad.is_null() {
                appstate.gamepad = SDL_OpenGamepad(event.gdevice.which);
                if appstate.gamepad.is_null() {
                    eprintln!(
                        "Failed to open gamepad ID {}: {}",
                        event.gdevice.which.0,
                        CStr::from_ptr(SDL_GetError()).to_string_lossy()
                    );
                }
            }
        },
        SDL_EVENT_GAMEPAD_REMOVED => unsafe {
            if !appstate.gamepad.is_null()
                && (SDL_GetGamepadID(appstate.gamepad) == event.gdevice.which.0)
            {
                SDL_CloseGamepad(appstate.gamepad);
                appstate.gamepad = ptr::null_mut();
            }
        },
        _ => {}
    }
    SDL_APP_CONTINUE
}

extern "C" fn appquit(appstate: *mut c_void, _result: SDL_AppResult) {
    if !appstate.is_null() {
        unsafe {
            let mut smw = Box::from_raw(appstate.cast::<Smw<AppState>>());
            let appstate = &mut smw.c;
            SDL_DestroyWindow(appstate.window);
        }
    }
}

extern "C" fn main_function(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        SDL_EnterAppMainCallbacks(
            argc,
            argv,
            Some(appinit),
            Some(appiter),
            Some(appevent),
            Some(appquit),
        )
    }
}

fn main() -> ExitCode {
    unsafe { (SDL_RunApp(0, ptr::null_mut(), Some(main_function), ptr::null_mut()) as u8).into() }
}

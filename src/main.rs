use std::{
    ffi::{CStr, c_char, c_int, c_uint, c_void},
    fs,
    process::ExitCode,
    ptr,
};

use sdl3_sys::{
    audio::{
        SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK, SDL_AUDIO_S16, SDL_AudioSpec, SDL_AudioStream,
        SDL_GetAudioStreamQueued, SDL_OpenAudioDeviceStream, SDL_PutAudioStreamData,
        SDL_ResumeAudioStreamDevice,
    },
    error::SDL_GetError,
    events::{
        SDL_EVENT_GAMEPAD_ADDED, SDL_EVENT_GAMEPAD_BUTTON_DOWN, SDL_EVENT_GAMEPAD_BUTTON_UP,
        SDL_EVENT_GAMEPAD_REMOVED, SDL_EVENT_QUIT, SDL_Event, SDL_EventType,
    },
    gamepad::{
        SDL_CloseGamepad, SDL_GAMEPAD_BUTTON_BACK, SDL_GAMEPAD_BUTTON_DPAD_DOWN,
        SDL_GAMEPAD_BUTTON_DPAD_LEFT, SDL_GAMEPAD_BUTTON_DPAD_RIGHT, SDL_GAMEPAD_BUTTON_DPAD_UP,
        SDL_GAMEPAD_BUTTON_EAST, SDL_GAMEPAD_BUTTON_LEFT_SHOULDER, SDL_GAMEPAD_BUTTON_NORTH,
        SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER, SDL_GAMEPAD_BUTTON_SOUTH, SDL_GAMEPAD_BUTTON_START,
        SDL_GAMEPAD_BUTTON_WEST, SDL_Gamepad, SDL_GamepadButton, SDL_GetGamepadButton,
        SDL_GetGamepadID, SDL_OpenGamepad,
    },
    init::{
        SDL_APP_CONTINUE, SDL_APP_FAILURE, SDL_APP_SUCCESS, SDL_AppResult, SDL_INIT_AUDIO,
        SDL_INIT_GAMEPAD, SDL_INIT_VIDEO, SDL_Init, SDL_SetAppMetadata,
    },
    log::SDL_Log,
    main::{SDL_EnterAppMainCallbacks, SDL_RunApp},
    pixels::SDL_PIXELFORMAT_XRGB8888,
    rect::SDL_Rect,
    surface::{
        SDL_BlitSurfaceScaled, SDL_CreateSurface, SDL_LockSurface, SDL_MUSTLOCK,
        SDL_SCALEMODE_NEAREST, SDL_Surface, SDL_UnlockSurface,
    },
    video::{
        SDL_CreateWindow, SDL_DestroyWindow, SDL_GetWindowSurface, SDL_UpdateWindowSurface,
        SDL_Window, SDL_WindowFlags,
    },
};
use smw::{Bind, SfcCallbacks, Smw, sfc_init, sfc_iter, sfc_quit};

struct Callbacks {}

impl Bind for Callbacks {}

struct AppState {
    pub window: *mut SDL_Window,
    pub surface: *mut SDL_Surface,
    pub stream: *mut SDL_AudioStream,
    pub gamepad: *mut SDL_Gamepad,
    pub gamepad_state: u16,
    pub smw: Smw,
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

        // setup callbacks
        const CLBK: SfcCallbacks = SfcCallbacks {
            video_refresh: Some({
                extern "C" fn video_refresh(
                    user: *mut c_void,
                    pixels: *const c_void,
                    width: c_uint,
                    height: c_uint,
                    pitch: usize,
                ) {
                    let appstate = unsafe { &mut *user.cast::<AppState>() };
                    let surface = unsafe { &mut *appstate.surface };

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
                                    *pixels
                                        .cast::<u32>()
                                        .add(y * pitch / size_of::<u32>() + x - 46)
                                        | 0xff000000;
                            }
                        }
                        if SDL_MUSTLOCK(surface) {
                            SDL_UnlockSurface(surface);
                        }

                        let window_surface = &mut *SDL_GetWindowSurface(appstate.window);
                        SDL_BlitSurfaceScaled(
                            surface,
                            ptr::null(),
                            window_surface,
                            ptr::null(),
                            SDL_SCALEMODE_NEAREST,
                        );
                        SDL_UpdateWindowSurface(appstate.window);
                    }
                }
                video_refresh
            }),
            audio_sample_batch: Some({
                extern "C" fn audio_sample_batch(
                    user: *mut c_void,
                    data: *const i16,
                    frames: usize,
                ) -> usize {
                    let appstate = unsafe { &mut *user.cast::<AppState>() };
                    unsafe {
                        SDL_PutAudioStreamData(
                            appstate.stream,
                            data.cast(),
                            (frames * size_of::<i16>() * 2) as i32,
                        );
                    }
                    frames
                }
                audio_sample_batch
            }),
            input_poll: Some({
                extern "C" fn input_poll(user: *mut c_void) {
                    let appstate = unsafe { &mut *user.cast::<AppState>() };

                    // poll the buttons we care about
                    if !appstate.gamepad.is_null() {
                        unsafe {
                            appstate.gamepad_state =
                                ((SDL_GetGamepadButton(appstate.gamepad, SDL_GAMEPAD_BUTTON_SOUTH)
                                    as u16)
                                    << 0)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_WEST,
                                    ) as u16)
                                        << 1)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_BACK,
                                    ) as u16)
                                        << 2)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_START,
                                    ) as u16)
                                        << 3)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_DPAD_UP,
                                    ) as u16)
                                        << 4)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_DPAD_DOWN,
                                    ) as u16)
                                        << 5)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_DPAD_LEFT,
                                    ) as u16)
                                        << 6)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_DPAD_RIGHT,
                                    ) as u16)
                                        << 7)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_EAST,
                                    ) as u16)
                                        << 8)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_NORTH,
                                    ) as u16)
                                        << 9)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_LEFT_SHOULDER,
                                    ) as u16)
                                        << 10)
                                    | ((SDL_GetGamepadButton(
                                        appstate.gamepad,
                                        SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER,
                                    ) as u16)
                                        << 11);
                        }
                    } else {
                        appstate.gamepad_state = 0;
                    }
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
                    let appstate = unsafe { &mut *user.cast::<AppState>() };
                    if port == 0 && device == 1 && index == 0 {
                        ((appstate.gamepad_state & (1 << id)) != 0) as i16
                    } else {
                        0
                    }
                }
                input_state
            }),
        };

        // load game
        let game = fs::read("smw.sfc").unwrap();

        *appstate = Box::into_raw(Box::new(AppState {
            window,
            surface,
            stream,
            gamepad: ptr::null_mut(),
            gamepad_state: 0,
            smw: Smw::new(Box::new(Callbacks {})),
        }))
        .cast();

        // init bsnes-mercury
        sfc_init(*appstate, &CLBK, game.as_ptr(), game.len());

        SDL_APP_CONTINUE
    }
}

extern "C" fn appiter(appstate: *mut c_void) -> SDL_AppResult {
    let appstate = unsafe { &mut *appstate.cast::<AppState>() };

    // do we need more audio?
    if unsafe { SDL_GetAudioStreamQueued(appstate.stream) < 0x1000 } {
        // run the game loop to get more audio
        // this synchronizes audio and video for us :)
        unsafe {
            sfc_iter((&raw mut *appstate).cast());
        }
    }

    SDL_APP_CONTINUE
}

extern "C" fn appevent(appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
    let appstate = unsafe { &mut *appstate.cast::<AppState>() };
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
            let mut appstate = Box::from_raw(appstate.cast::<AppState>());
            sfc_quit((&raw mut *appstate).cast());
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

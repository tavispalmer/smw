use std::{
    ffi::{CStr, c_char, c_int, c_void},
    mem::MaybeUninit,
    process::ExitCode,
    ptr,
};

use sdl3_sys::{
    error::SDL_GetError,
    events::{
        SDL_EVENT_GAMEPAD_ADDED, SDL_EVENT_GAMEPAD_BUTTON_DOWN, SDL_EVENT_GAMEPAD_BUTTON_UP,
        SDL_EVENT_GAMEPAD_REMOVED, SDL_EVENT_QUIT, SDL_Event, SDL_EventType,
    },
    gamepad::{
        SDL_CloseGamepad, SDL_GAMEPAD_BUTTON_BACK, SDL_GAMEPAD_BUTTON_DPAD_DOWN,
        SDL_GAMEPAD_BUTTON_DPAD_LEFT, SDL_GAMEPAD_BUTTON_DPAD_RIGHT, SDL_GAMEPAD_BUTTON_DPAD_UP,
        SDL_GAMEPAD_BUTTON_EAST, SDL_GAMEPAD_BUTTON_GUIDE, SDL_GAMEPAD_BUTTON_LEFT_SHOULDER,
        SDL_GAMEPAD_BUTTON_NORTH, SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER, SDL_GAMEPAD_BUTTON_SOUTH,
        SDL_GAMEPAD_BUTTON_START, SDL_GAMEPAD_BUTTON_WEST, SDL_Gamepad, SDL_GamepadButton,
        SDL_GetGamepadID, SDL_OpenGamepad,
    },
    init::{
        SDL_APP_CONTINUE, SDL_APP_FAILURE, SDL_APP_SUCCESS, SDL_AppResult, SDL_INIT_GAMEPAD,
        SDL_INIT_VIDEO, SDL_Init, SDL_SetAppMetadata,
    },
    log::SDL_Log,
    main::{SDL_EnterAppMainCallbacks, SDL_RunApp},
    render::{
        SDL_CreateWindowAndRenderer, SDL_DestroyRenderer, SDL_LOGICAL_PRESENTATION_LETTERBOX,
        SDL_Renderer, SDL_SetRenderLogicalPresentation,
    },
    video::{SDL_DestroyWindow, SDL_Window, SDL_WindowFlags},
};
use smw::{Bind, Smw};

struct Callbacks {}

impl Bind for Callbacks {}

struct AppState {
    pub window: *mut SDL_Window,
    pub renderer: *mut SDL_Renderer,
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

        if !SDL_Init(SDL_INIT_VIDEO | SDL_INIT_GAMEPAD) {
            SDL_Log(c"Couldn't initialize SDL: %s".as_ptr(), SDL_GetError());
            return SDL_APP_FAILURE;
        }

        let mut window = MaybeUninit::uninit();
        let mut renderer = MaybeUninit::uninit();
        if !SDL_CreateWindowAndRenderer(
            c"smw".as_ptr(),
            800,
            600,
            SDL_WindowFlags(0),
            window.as_mut_ptr(),
            renderer.as_mut_ptr(),
        ) {
            return SDL_APP_FAILURE;
        }
        let window = window.assume_init();
        let renderer = renderer.assume_init();

        SDL_SetRenderLogicalPresentation(renderer, 800, 600, SDL_LOGICAL_PRESENTATION_LETTERBOX);

        *appstate = Box::into_raw(Box::new(AppState {
            window,
            renderer,
            gamepad: ptr::null_mut(),
            gamepad_state: 0,
            smw: Smw::new(Box::new(Callbacks {})),
        }))
        .cast();

        SDL_APP_CONTINUE
    }
}

extern "C" fn appiter(appstate: *mut c_void) -> SDL_AppResult {
    let appstate = unsafe { &mut *appstate.cast::<AppState>() };

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
                appstate.gamepad_state = 0;
                SDL_CloseGamepad(appstate.gamepad);
                appstate.gamepad = ptr::null_mut();
            }
        },
        SDL_EVENT_GAMEPAD_BUTTON_DOWN => unsafe {
            if SDL_GetGamepadID(appstate.gamepad) == event.gbutton.which.0 {
                match SDL_GamepadButton(event.gbutton.button as i32) {
                    SDL_GAMEPAD_BUTTON_SOUTH => appstate.gamepad_state |= 0b0000_0000_0001,
                    SDL_GAMEPAD_BUTTON_WEST => appstate.gamepad_state |= 0b0000_0000_0010,
                    SDL_GAMEPAD_BUTTON_BACK => appstate.gamepad_state |= 0b0000_0000_0100,
                    SDL_GAMEPAD_BUTTON_START => appstate.gamepad_state |= 0b0000_0000_1000,
                    SDL_GAMEPAD_BUTTON_DPAD_UP => appstate.gamepad_state |= 0b0000_0001_0000,
                    SDL_GAMEPAD_BUTTON_DPAD_DOWN => appstate.gamepad_state |= 0b0000_0010_0000,
                    SDL_GAMEPAD_BUTTON_DPAD_LEFT => appstate.gamepad_state |= 0b0000_0100_0000,
                    SDL_GAMEPAD_BUTTON_DPAD_RIGHT => appstate.gamepad_state |= 0b0000_1000_0000,
                    SDL_GAMEPAD_BUTTON_EAST => appstate.gamepad_state |= 0b0001_0000_0000,
                    SDL_GAMEPAD_BUTTON_NORTH => appstate.gamepad_state |= 0b0010_0000_0000,
                    SDL_GAMEPAD_BUTTON_LEFT_SHOULDER => appstate.gamepad_state |= 0b0100_0000_0000,
                    SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER => appstate.gamepad_state |= 0b1000_0000_0000,
                    _ => {}
                }
            }
        },
        SDL_EVENT_GAMEPAD_BUTTON_UP => unsafe {
            if SDL_GetGamepadID(appstate.gamepad) == event.gbutton.which.0 {
                match SDL_GamepadButton(event.gbutton.button as i32) {
                    SDL_GAMEPAD_BUTTON_SOUTH => appstate.gamepad_state &= !0b0000_0000_0001,
                    SDL_GAMEPAD_BUTTON_WEST => appstate.gamepad_state &= !0b0000_0000_0010,
                    SDL_GAMEPAD_BUTTON_BACK => appstate.gamepad_state &= !0b0000_0000_0100,
                    SDL_GAMEPAD_BUTTON_START => appstate.gamepad_state &= !0b0000_0000_1000,
                    SDL_GAMEPAD_BUTTON_DPAD_UP => appstate.gamepad_state &= !0b0000_0001_0000,
                    SDL_GAMEPAD_BUTTON_DPAD_DOWN => appstate.gamepad_state &= !0b0000_0010_0000,
                    SDL_GAMEPAD_BUTTON_DPAD_LEFT => appstate.gamepad_state &= !0b0000_0100_0000,
                    SDL_GAMEPAD_BUTTON_DPAD_RIGHT => appstate.gamepad_state &= !0b0000_1000_0000,
                    SDL_GAMEPAD_BUTTON_EAST => appstate.gamepad_state &= !0b0001_0000_0000,
                    SDL_GAMEPAD_BUTTON_NORTH => appstate.gamepad_state &= !0b0010_0000_0000,
                    SDL_GAMEPAD_BUTTON_LEFT_SHOULDER => appstate.gamepad_state &= !0b0100_0000_0000,
                    SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER => {
                        appstate.gamepad_state &= !0b1000_0000_0000
                    }
                    _ => {}
                }
            }
        },
        _ => {}
    }
    SDL_APP_CONTINUE
}

extern "C" fn appquit(appstate: *mut c_void, _result: SDL_AppResult) {
    if !appstate.is_null() {
        unsafe {
            let appstate = Box::from_raw(appstate.cast::<AppState>());
            SDL_DestroyRenderer(appstate.renderer);
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

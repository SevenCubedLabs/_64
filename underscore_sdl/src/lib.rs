#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[allow(dead_code)]
mod bindings {
    mod c_types {
        pub type c_short = i16;
        pub type c_int = i32;
        pub type c_schar = i8;
        pub type c_char = i8;
        pub type c_uchar = u8;
        pub type c_ushort = u16;
        pub type c_uint = u32;
        pub type c_long = i64;
        pub type c_longlong = i64;
        pub type c_ulong = u64;
        pub type c_ulonglong = i64;
        pub type c_void = core::ffi::c_void;
    }

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;

#[derive(Debug, Copy, Clone)]
pub enum Event {
    Quit,
    Keyboard {
        down: bool,
        timestamp: u32,
        sym: i32,
        mod_: u32,
    },
    TextInput {
        text: [i8; 32],
    },
}

pub struct EventFeed;

impl EventFeed {
    pub fn text_input(&self, enable: bool) {
        unsafe {
            if enable {
                SDL_StartTextInput();
            } else {
                SDL_StopTextInput();
            }
        }
    }
}

impl Iterator for EventFeed {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut event = core::mem::zeroed();

            match SDL_PollEvent(&mut event) {
                1 => match event.type_ {
                    SDL_QUIT => Some(Event::Quit),
                    SDL_KEYDOWN | SDL_KEYUP => {
                        let SDL_KeyboardEvent {
                            type_,
                            timestamp,
                            keysym: SDL_Keysym { sym, mod_, .. },
                            ..
                        } = event.key;

                        let down = match type_ {
                            SDL_KEYDOWN => true,
                            SDL_KEYUP => false,
                            _ => panic!(),
                        };

                        Some(Event::Keyboard {
                            down,
                            timestamp,
                            sym,
                            mod_: mod_ as _,
                        })
                    }

                    SDL_TEXTINPUT => {
                        let SDL_TextInputEvent { text, .. } = event.text;

                        Some(Event::TextInput { text: text })
                    }
                    _ => None,
                },
                _ => None,
            }
        }
    }
}

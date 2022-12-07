use log;
use sdl2_sys::*;

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
    pub fn new() -> Self {
        unsafe {
            SDL_InitSubSystem(SDL_INIT_EVENTS);
        }

        Self {}
    }

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
            log::trace!("polling event");
            let mut event = core::mem::MaybeUninit::uninit();
            if SDL_PollEvent(event.as_mut_ptr()) == 1 {
                let event = event.assume_init();
                log::trace!("event type {:?} receieved", event.type_);
                match core::mem::transmute(event.type_) {
                    SDL_EventType::SDL_QUIT => Some(Event::Quit),
                    SDL_EventType::SDL_KEYDOWN | SDL_EventType::SDL_KEYUP => {
                        let SDL_KeyboardEvent {
                            type_,
                            timestamp,
                            keysym: SDL_Keysym { sym, mod_, .. },
                            ..
                        } = event.key;

                        let down = match core::mem::transmute(type_) {
                            SDL_EventType::SDL_KEYDOWN => true,
                            SDL_EventType::SDL_KEYUP => false,
                            _ => panic!(),
                        };

                        Some(Event::Keyboard {
                            down,
                            timestamp,
                            sym,
                            mod_: mod_ as _,
                        })
                    }

                    SDL_EventType::SDL_TEXTINPUT => {
                        let SDL_TextInputEvent { text, .. } = event.text;

                        Some(Event::TextInput { text: text })
                    }

                    _ => Option::None,
                }
            } else {
                Option::None
            }
        }
    }
}

impl Drop for EventFeed {
    fn drop(&mut self) {
        unsafe {
            SDL_QuitSubSystem(SDL_INIT_EVENTS);
        }
    }
}

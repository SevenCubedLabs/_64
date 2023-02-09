use crate::sdl::*;

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

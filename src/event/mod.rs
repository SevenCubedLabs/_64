use crate::sys::*;

pub struct EventFeed;
pub enum Event {
    Quit,
    UnImp { type_: u32 },
}

impl Iterator for EventFeed {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut event = core::mem::zeroed();

            match SDL_PollEvent(&mut event) {
                1 => Some(match event.type_ {
                    SDL_QUIT => Event::Quit,
                    x => Event::UnImp { type_: x },
                }),
                _ => None,
            }
        }
    }
}

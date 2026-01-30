#![no_std]
pub mod clock;

use core::convert::Infallible;
use embedded_graphics::mono_font::MonoFont;
use profont::PROFONT_18_POINT;
use ratatui::prelude::*;

use crate::clock::{Clock, TimeWidget};
extern crate alloc;

/// Regular font.
pub const FONT_REGULAR: MonoFont<'static> = PROFONT_18_POINT;
/// Bold font.
pub const FONT_BOLD: Option<MonoFont<'static>> = None;
/// Italic font.
pub const FONT_ITALIC: Option<MonoFont<'static>> = None;

pub enum MemoriState {
    Time(Clock),
}

impl Default for MemoriState {
    fn default() -> Self {
        MemoriState::Time(Clock::new())
    }
}

pub struct Memori<B: Backend> {
    term: Terminal<B>,
}

impl<B> Memori<B>
where
    B: Backend,
{
    pub fn new(term: Terminal<B>) -> Self {
        Self { term }
    }

    pub fn update(&mut self, state: &mut MemoriState) -> Result<(), Infallible> {
        use MemoriState::*;
        self.term
            .draw(|f| {
                let area = f.area();

                match state {
                    Time(clock) => {
                        let widget = TimeWidget;
                        f.render_stateful_widget(widget, area, clock);
                    }
                }
            })
            .expect("render callback should not fail");

        Ok(())
    }
}

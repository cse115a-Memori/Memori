#![no_std]

use alloc::{format};
use embedded_graphics::mono_font::MonoFont;
use profont::PROFONT_18_POINT;
use ratatui::prelude::*;

use crate::name::Name;
use crate::clock::Clock;
extern crate alloc;

pub mod name;
pub mod clock;

/// Regular font.
pub const FONT_REGULAR: MonoFont<'static> = PROFONT_18_POINT;
/// Bold font.
pub const FONT_BOLD: Option<MonoFont<'static>> = None;
/// Italic font.
pub const FONT_ITALIC: Option<MonoFont<'static>> = None;

pub enum MemoriState {
    Example(Counter),
    Name(Name),
    Clock(Clock),
    // Clo(Clo)
}

impl Default for MemoriState {
    fn default() -> Self {
        MemoriState::Example(Counter { i: 0 })
    }
}

pub struct Counter {
    pub i: u32,
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

    pub fn update(&mut self, state: &MemoriState) -> Result<(), B::Error> {
        self.term
            .draw(|f| {
                f.render_widget(state, f.area());
            })
            .map(|_| ())
    }
}

impl Widget for &MemoriState {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self {
            MemoriState::Example(counter) => {
                let string = format!("Hello Suri! {}", counter.i);
                Text::from(string).render(area, buf);
            }
            MemoriState::Name(name) => {
                name.render(area, buf);
            }
            MemoriState::Clock(clock) => {
                clock.render(area, buf);
            }
        }
    }
}

#![no_std]

use embedded_graphics::mono_font::MonoFont;
use embedded_graphics_unicodefonts::*;
use ratatui::prelude::*;

extern crate alloc;

pub mod layout;
pub mod widgets;

mod state;
pub use state::*;

/// Regular font.
pub const FONT_REGULAR: MonoFont<'static> = MONO_6X10;
/// Bold font.
pub const FONT_BOLD: Option<MonoFont<'static>> = None;
/// Italic font.
pub const FONT_ITALIC: Option<MonoFont<'static>> = None;

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

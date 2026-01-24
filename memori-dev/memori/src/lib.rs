#![no_std]

use alloc::format;
use core::convert::Infallible;
use embedded_graphics::mono_font::MonoFont;
use profont::PROFONT_18_POINT;
use ratatui::prelude::*;
extern crate alloc;

/// Regular font.
pub const FONT_REGULAR: MonoFont<'static> = PROFONT_18_POINT;
/// Bold font.
pub const FONT_BOLD: Option<MonoFont<'static>> = None;
/// Italic font.
pub const FONT_ITALIC: Option<MonoFont<'static>> = None;


pub enum MemoriState {
    Example(Counter),
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

    pub fn update(&mut self, state: &MemoriState) -> Result<(), Infallible> {
        use MemoriState::*;
        let widget = match state {
            Example(state) => {
                let string = format!("Hello Suri! {}", state.i);
                Text::from(string)
            }
        };

        self.term
            .draw(|f| {
                f.render_widget(widget, f.area());
            })
            .expect("render callback should not fail");
        Ok(())
    }
}

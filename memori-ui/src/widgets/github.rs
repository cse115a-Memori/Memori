use crate::alloc::string::ToString;
use alloc::format;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::prelude::Stylize;
use alloc::string::String;
use ratatui::widgets::{Block, Borders, Widget};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Github {
    pub username: String,       
    pub repo: Option<String>, 
    
    // Cached stats (updated periodically)
    pub open_issues: u32,
    pub open_prs: u32,
    pub stars: u32,                 
    pub notifications: u32,         
}

impl Default for Github {
    fn default() -> Self {
        Self::new("CaiNann".to_string(), None)
    }
}

impl Github {
    pub fn new(username: String, repo: Option<String>) -> Self {
        Self {
            username,
            repo,
            open_issues: 0,
            open_prs: 0,
            stars: 0,
            notifications: 0,
        }
    }
}

impl Widget for &Github {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let body = match self.repo {
            Some(ref repo) => format!("{}\n{}\n{}\n{}\n{}\n{}", 
                self.username, 
                repo, 
                self.open_issues, 
                self.open_prs, 
                self.stars, 
                self.notifications),
            None => self.username.clone(),
        };

        let border_set = border::PLAIN;
        
        // Render the block with borders
        let block = Block::default()
            .title(Line::from(" Github ").bold().centered())
            .borders(Borders::ALL)
            .border_set(border_set)
            .border_style(ratatui::style::Style::default()
                .fg(ratatui::style::Color::White)
            );
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Calculate center position
        let text_len = body.len() as u16;
        let center_x = inner_area.x + (inner_area.width.saturating_sub(text_len)) / 2;
        let center_y = inner_area.y + inner_area.height / 2;

        // Render the centered text
        buf.set_string(center_x, center_y, body, Style::default());
    }
}

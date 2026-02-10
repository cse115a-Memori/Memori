use crate::alloc::string::ToString;
use alloc::format;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Alignment, Layout, Direction, Constraint};
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::prelude::Stylize;
use alloc::string::String;
use ratatui::widgets::{Block, Borders, Widget, Paragraph, Padding};
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
        let border_set = border::PLAIN;
                
        // Outer "Github" box
        let outer_block = Block::default()
            .title(Line::from(" Github ").bold().centered())
            .borders(Borders::ALL)
            .border_set(border_set)
            .border_style(Style::default().fg(ratatui::style::Color::White));
        
        let outer_inner = outer_block.inner(area);
        outer_block.render(area, buf);
        
        // Determine layout based on available space
        // Full screen is ~296x128, half vertical is ~148x128, half horizontal is ~296x64
        match (outer_inner.width, outer_inner.height) {
            // Small height, fourths or horizontal splits 
            (w, h) if h < 6 => {
                if let Some(ref repo) = self.repo {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Percentage(50),  // Left half
                            Constraint::Percentage(50),  // Right half
                        ])
                        .split(outer_inner);
                    
                    // Left half: username and repo
                    let left_text = format!("{}\n({})", self.username, repo);
                    let text_height = 2;  // 2 lines
                    let vertical_padding = (chunks[0].height.saturating_sub(text_height)) / 2;
                    
                    let left_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(vertical_padding),
                            Constraint::Length(text_height),
                            Constraint::Min(0),
                        ])
                        .split(chunks[0]);
                    
                    Paragraph::new(left_text)
                        .alignment(Alignment::Center)
                        .render(left_chunks[1], buf);
                            
                    // Right half: stats list
                    let stats = format!(
                        "Issues: {}\nPRs: {}\nStars: {}\nNotifs: {}",
                        self.open_issues,
                        self.open_prs,
                        self.stars,
                        self.notifications
                    );
                    Paragraph::new(stats)
                        .alignment(Alignment::Left)
                        .render(chunks[1], buf);
                } else {
                    Paragraph::new(self.username.clone())
                        .alignment(Alignment::Center)
                        .render(outer_inner, buf);
                }
            },
            
            // Half vertical (narrow but tall) - stack everything vertically
            (w, _h) if w < 30 => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(2),  // Username + space
                        Constraint::Min(0),     // Repo box
                    ])
                    .split(outer_inner);
                
                buf.set_string(chunks[0].x, chunks[0].y, &self.username, Style::default());
                
                if let Some(ref repo) = self.repo {
                    let repo_block = Block::default()
                        .title(Line::from(format!(" {} ", repo)))
                        .borders(Borders::ALL)
                        .border_set(border_set)
                        .padding(Padding::new(1, 1, 1, 1));
                    
                    let repo_inner = repo_block.inner(chunks[1]);
                    repo_block.render(chunks[1], buf);
                    
                    // Compact stats for narrow space
                    let stats = format!(
                        "Issues: {}\nPRs: {}\nStars: {}\nNotifs: {}",
                        self.open_issues, self.open_prs,
                        self.stars, self.notifications
                    );
                    
                    Paragraph::new(stats)
                        .alignment(Alignment::Left)
                        .render(repo_inner, buf);
                }
            },
            
            // Full screen or large - full nested layout
            _ => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(2),  // Username + space
                        Constraint::Min(0),     // Repo box
                    ])
                    .split(outer_inner);
                
                buf.set_string(chunks[0].x, chunks[0].y, &self.username, Style::default());
                
                if let Some(ref repo) = self.repo {
                    let repo_block = Block::default()
                        .title(Line::from(format!(" {} ", repo)))
                        .borders(Borders::ALL)
                        .border_set(border_set)
                        .border_style(Style::default().fg(ratatui::style::Color::White));
                    
                    let repo_inner = repo_block.inner(chunks[1]);
                    repo_block.render(chunks[1], buf);
                    
                    let stats = format!(
                        "Issues: {}\nPRs: {}\nStars: {}\nNotifs: {}",
                        self.open_issues, self.open_prs,
                        self.stars, self.notifications
                    );
                    
                    Paragraph::new(stats)
                        .alignment(Alignment::Left)
                        .render(repo_inner, buf);
                }
            }
        }
    }
}

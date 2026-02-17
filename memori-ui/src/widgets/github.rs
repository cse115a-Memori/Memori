use crate::alloc::string::ToString;
use alloc::format;
use alloc::vec::Vec;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Alignment, Layout, Direction, Constraint};
use ratatui::widgets::{BarGroup, Bar, BarChart};
use ratatui::style::{Style, Color};
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
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
    pub commits: [i32; 7],
    weekday: usize,
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
            commits: [0, 10, 0, 8, 5, 7, 0],
            weekday: 3,
        }
    }
}

fn build_commit_graph(commits: &[i32; 7], today_weekday: usize) -> BarChart<'static> {
    let days = ["M", "T", "W", "T", "F", "S", "S"];
    let bars: Vec<Bar> = commits
        .iter()
        .enumerate()
        .map(|(i, &count)| {
            let label = days[(today_weekday + i + 1) % 7];
            Bar::default()
                .value(count as u64)
                .label(Line::from(label))
                .value_style(Style::default().fg(Color::White))
                .style(Style::default().fg(Color::DarkGray))
        })
        .collect();

    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .bar_width(1)
        .bar_gap(2)
        .value_style(Style::default().fg(Color::White))
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
            
            (w, h) if w < 30 && h < 6 => {
                if let Some(ref repo) = self.repo {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Percentage(50),  // username and repo 
                            Constraint::Percentage(50),  // Stat list 
                        ])
                        .split(outer_inner);
                    
                    // Left half: username and repo
                    let left_text = format!("{}\n({})", self.username, repo);
                    Paragraph::new(left_text)
                        .alignment(Alignment::Center)
                        .render(chunks[0], buf);
                    
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
            
            (w, h) if h < 6 => {
                if let Some(ref repo) = self.repo {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Percentage(25),  // username and repo 
                            Constraint::Percentage(50),  // Commit graph 
                            Constraint::Percentage(25),  // other stats 
                        ])
                        .split(outer_inner);
                    
                    // Left half: username and repo
                    let left_text = format!("{}\n({})", self.username, repo);
                    Paragraph::new(left_text)
                        .alignment(Alignment::Center)
                        .render(chunks[0], buf);
                    
                    // Middle: commit graph
                    build_commit_graph(&self.commits, self.weekday)
                                .render(chunks[1], buf);
                            
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
                        .render(chunks[2], buf);
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
                        Constraint::Length(1),  // Username + space
                        Constraint::Length(2), // Commit graph 
                        Constraint::Min(0),     // Repo box
                    ])
                    .split(outer_inner);
                
                buf.set_string(chunks[0].x, chunks[0].y, &self.username, Style::default());
                
                // Render graph
                build_commit_graph(&self.commits, self.weekday).render(chunks[1], buf);
                
                if let Some(ref repo) = self.repo {
                    let repo_block = Block::default()
                        .title(Line::from(format!(" {} ", repo)))
                        .borders(Borders::ALL)
                        .border_set(border_set)
                        .padding(Padding::new(1, 1, 0, 0));
                    
                    let repo_inner = repo_block.inner(chunks[2]);
                    repo_block.render(chunks[2], buf);
                    
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

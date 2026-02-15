use alloc::vec::Vec;
use hashbrown::HashMap;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};
use serde::{Deserialize, Serialize};

use crate::{
    layout::MemoriLayout,
    widgets::{MemoriWidget, Name, WidgetId, WidgetKind, UpdateFrequency},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoriState {
    active_frame_idx: usize,
    pub widgets: HashMap<WidgetId, MemoriWidget>,
    frames: Vec<MemoriLayout>,
    #[allow(dead_code)]
    frame_time: u32,
}

impl Default for MemoriState {
    fn default() -> Self {
        let mut widgets = HashMap::new();

        let widget_id = WidgetId(0);

        let widget = MemoriWidget::new(
            widget_id,
            WidgetKind::Name(Name::new("Surendra")),
            UpdateFrequency::Never,
            UpdateFrequency::Never,
        );

        widgets.insert(widget_id, widget);

        let frames = alloc::vec![MemoriLayout::Full(widget_id)];

        Self {
            active_frame_idx: 0,
            frames,
            widgets,
            frame_time: 5,
        }
    }
}

impl MemoriState {
    pub fn new(
        active_frame_idx: usize,
        widgets: impl IntoIterator<Item = MemoriWidget>,
        frames: Vec<MemoriLayout>,
        frame_time: u32,
    ) -> Self {
        // sanity check
        if active_frame_idx >= frames.len() {
            panic!("active_frame_idx overflows frames.len()")
        }

        Self {
            active_frame_idx,
            widgets: widgets.into_iter().map(|w| (w.id, w)).collect(),
            frames,
            frame_time,
        }
    }

    pub fn active_frame(&self) -> &MemoriLayout {
        self.frames
            .get(self.active_frame_idx)
            .expect("invariant failure! active_frame_idx is not a index into frames!")
    }
}
impl Widget for &MemoriState {
    //TODO: remove after we finish this
    #[allow(unused)]
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.active_frame() {
            MemoriLayout::Full(id) => self
                .widgets
                .get(id)
                .expect("WidgetId should exist in hashmap")
                .render(area, buf),
            MemoriLayout::VSplit { left, right } => {
                let rects =
                    Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(area);
                self.widgets
                    .get(left)
                    .expect("WidgetId should exist in hashmap")
                    .render(rects[0], buf);
                self.widgets
                    .get(right)
                    .expect("WidgetId should exist in hashmap")
                    .render(rects[1], buf);
            }
            MemoriLayout::HSplit { top, bottom } => {
                let rects =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(area);
                self.widgets
                    .get(top)
                    .expect("WidgetId should exist in hashmap")
                    .render(rects[0], buf);
                self.widgets
                    .get(bottom)
                    .expect("WidgetId should exist in hashmap")
                    .render(rects[1], buf);
            }
            MemoriLayout::VSplitWithRightHSplit {
                left,
                right_top,
                right_bottom,
            } => {
                let vertical_rects =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(area);
                self.widgets
                    .get(left)
                    .expect("WidgetId should exist in hashmap")
                    .render(vertical_rects[0], buf);

                let right_rects =
                    Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(vertical_rects[1]);
                self.widgets
                    .get(right_top)
                    .expect("WidgetId should exist in hashmap")
                    .render(right_rects[0], buf);
                self.widgets
                    .get(right_bottom)
                    .expect("WidgetId should exist in hashmap")
                    .render(right_rects[1], buf);
            }
            MemoriLayout::HSplitWithTopVSplit {
                bottom,
                top_right,
                top_left,
            } => {
                let horizontal_rects =
                    Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(area);

                let top_rects =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(horizontal_rects[0]);

                self.widgets
                    .get(top_left)
                    .expect("WidgetId should exist in hashmap")
                    .render(top_rects[0], buf);
                self.widgets
                    .get(top_right)
                    .expect("WidgetId should exist in hashmap")
                    .render(top_rects[1], buf);
                self.widgets
                    .get(bottom)
                    .expect("WidgetId should exist in hashmap")
                    .render(horizontal_rects[1], buf);
            }
            MemoriLayout::VSplitWithLeftHSplit {
                left_top,
                left_bottom,
                right,
            } => {
                let vertical_rects =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(area);

                let left_rects =
                    Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(vertical_rects[0]);
                self.widgets
                    .get(left_top)
                    .expect("WidgetId should exist in hashmap")
                    .render(left_rects[0], buf);
                self.widgets
                    .get(left_bottom)
                    .expect("WidgetId should exist in hashmap")
                    .render(left_rects[1], buf);

                self.widgets
                    .get(right)
                    .expect("WidgetId should exist in hashmap")
                    .render(vertical_rects[1], buf);
            }
            MemoriLayout::HSplitWithBottomVSplit {
                top,
                bottom_left,
                bottom_right,
            } => {
                let horizontal_rects =
                    Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(area);
                self.widgets
                    .get(top)
                    .expect("WidgetId should exist in hashmap")
                    .render(horizontal_rects[0], buf);

                let bottom_rects =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(horizontal_rects[1]);
                self.widgets
                    .get(bottom_left)
                    .expect("WidgetId should exist in hashmap")
                    .render(bottom_rects[0], buf);
                self.widgets
                    .get(bottom_right)
                    .expect("WidgetId should exist in hashmap")
                    .render(bottom_rects[1], buf);
            }
            MemoriLayout::Fourths {
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            } => {
                let horizontal_rects =
                    Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(area);

                let top_rects =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(horizontal_rects[0]);
                self.widgets
                    .get(top_left)
                    .expect("WidgetId should exist in hashmap")
                    .render(top_rects[0], buf);
                self.widgets
                    .get(top_right)
                    .expect("WidgetId should exist in hashmap")
                    .render(top_rects[1], buf);

                let bottom_rects =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(horizontal_rects[1]);
                self.widgets
                    .get(bottom_left)
                    .expect("WidgetId should exist in hashmap")
                    .render(bottom_rects[0], buf);
                self.widgets
                    .get(bottom_right)
                    .expect("WidgetId should exist in hashmap")
                    .render(bottom_rects[1], buf);
            }
        }
    }
}

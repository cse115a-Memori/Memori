#![allow(unused)]

use alloc::boxed::Box;
use ratatui::widgets::Widget;
use serde::{Deserialize, Serialize};

use crate::widgets::WidgetId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MemoriLayout {
    /// ┌─────────────────┐
    /// │                 │
    /// │                 │
    /// │      Full       │
    /// │                 │
    /// │                 │
    /// └─────────────────┘
    Full(WidgetId),

    /// ┌────────┬────────┐
    /// │        │        │
    /// │        │        │
    /// │  Left  │ Right  │
    /// │        │        │
    /// │        │        │
    /// └────────┴────────┘
    VSplit { left: WidgetId, right: WidgetId },

    /// ┌─────────────────┐
    /// │                 │
    /// │       Top       │
    /// │                 │
    /// ├─────────────────┤
    /// │                 │
    /// │     Bottom      │
    /// │                 │
    /// └─────────────────┘
    HSplit { top: WidgetId, bottom: WidgetId },

    /// ┌──────┬──────────┐
    /// │      │          │
    /// │      │   Right  │
    /// │      │    Top   │
    /// │ Left ├──────────┤
    /// │      │          │
    /// │      │  Right   │
    /// │      │  Bottom  │
    /// └──────┴──────────┘
    VSplitWithRightHSplit {
        left: WidgetId,
        right_top: WidgetId,
        right_bottom: WidgetId,
    },

    /// ┌────────┬────────┐
    /// │        │        │
    /// │  Top   │  Top   │
    /// │  Left  │ Right  │
    /// ├────────┴────────┤
    /// │                 │
    /// │                 │
    /// │     Bottom      │
    /// └─────────────────┘
    HSplitWithTopVSplit {
        bottom: WidgetId,
        top_right: WidgetId,
        top_left: WidgetId,
    },

    /// ┌──────────┬──────┐
    /// │          │      │
    /// │   Left   │      │
    /// │    Top   │      │
    /// ├──────────┤ Right│
    /// │          │      │
    /// │   Left   │      │
    /// │  Bottom  │      │
    /// └──────────┴──────┘
    VSplitWithLeftHSplit {
        left_top: WidgetId,
        left_bottom: WidgetId,
        right: WidgetId,
    },

    /// ┌─────────────────┐
    /// │                 │
    /// │                 │
    /// │       Top       │
    /// │                 │
    /// ├────────┬────────┤
    /// │        │        │
    /// │ Bottom │ Bottom │
    /// │  Left  │ Right  │
    /// └────────┴────────┘
    HSplitWithBottomVSplit {
        top: WidgetId,
        bottom_left: WidgetId,
        bottom_right: WidgetId,
    },

    /// ┌────────┬────────┐
    /// │        │        │
    /// │  Top   │  Top   │
    /// │  Left  │ Right  │
    /// ├────────┼────────┤
    /// │        │        │
    /// │ Bottom │ Bottom │
    /// │  Left  │ Right  │
    /// └────────┴────────┘
    Fourths {
        top_left: WidgetId,
        top_right: WidgetId,
        bottom_left: WidgetId,
        bottom_right: WidgetId,
    },
}

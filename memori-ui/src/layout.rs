#![allow(unused)]

use alloc::vec;
use alloc::vec::Vec;
use core as std;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::widgets::WidgetId;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[serde(rename_all_fields = "camelCase")]
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
    #[specta(rename_all = "camelCase")]
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
    #[specta(rename_all = "camelCase")]
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
    #[specta(rename_all = "camelCase")]
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
    #[specta(rename_all = "camelCase")]
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
    #[specta(rename_all = "camelCase")]
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
    #[specta(rename_all = "camelCase")]
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
    #[specta(rename_all = "camelCase")]
    Fourths {
        top_left: WidgetId,
        top_right: WidgetId,
        bottom_left: WidgetId,
        bottom_right: WidgetId,
    },
}

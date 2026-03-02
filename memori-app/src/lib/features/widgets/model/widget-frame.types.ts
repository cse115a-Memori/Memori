import type { LayoutVariant } from '@/features/widgets/model/layout'
import type { MemoriWidget } from '@/tauri'

export const GROUP_IDS = ['widgets', 'frame-widgets'] as const

export type GroupId = (typeof GROUP_IDS)[number]
export type WidgetDisplay = { name: string; content: string }
export type WidgetView = {
	id: string
	widgetId: Memori.WidgetId
	kind: MemoriWidget['kind']
}
export type GroupWidgets = Record<GroupId, WidgetView[]>

export type LayoutsInFrame = Record<LayoutVariant, GroupWidgets>

export type Frame = { activeLayout: LayoutVariant; frameLayouts: LayoutsInFrame }

export type FrameHash = Record<LayoutVariant, string>

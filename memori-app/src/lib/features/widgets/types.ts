import type { LayoutVariant } from '@/features/widgets/model/layout.ts'
import type { MemoriStateInput, MemoriWidget } from '@/tauri'

export type FrameDraft = {
	activeLayout: LayoutVariant
	layoutAssignments: Record<LayoutVariant, MemoriWidget[]>
}

export type WidgetsState = {
	activeFrameIdx: number
	frameTime: number
	widgets: MemoriWidget[]
	frames: FrameDraft[]
}

export type { MemoriStateInput }

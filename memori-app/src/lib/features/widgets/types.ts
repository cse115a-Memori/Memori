import type {
	WidgetFrameEntry,
	WidgetFrameSignatures,
} from '@/features/widgets/model/widget-frame.ts'
import type { MemoriStateInput, MemoriWidget } from '@/tauri'

export type WidgetsEditorState = {
	widgetPool: MemoriWidget[]
	widgetFrames: WidgetFrameEntry[]
	lastFlashedSignaturesByFrame: WidgetFrameSignatures[]
}

export type MemoriDraftState = {
	draft: MemoriStateInput | null
}

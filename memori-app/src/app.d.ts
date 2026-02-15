import type {
	MemoriWidget,
	MemoriLayout,
	WidgetKind as MemoriWidgetKind,
	WidgetId as MemoriWidgetId,
} from '@/tauri'

import type { LayoutSlot as MemoriLayoutSlot } from '@/widget-utils'

// helper: normalize raw payload -> always `{ widget: ... }` form
type NormalizeWidgetPayload<V> = V extends MemoriWidgetId
	? { center: MemoriWidgetId }
	: V extends Record<string, MemoriWidgetId>
		? V
		: never

type DerivedLayout<T> = T extends unknown
	? {
			[K in keyof T & string]: T[K] extends
				| MemoriWidgetId
				| Record<string, MemoriWidgetId>
				? { variant: K; widgets: NormalizeWidgetPayload<T[K]> }
				: never
		}[keyof T & string]
	: never

type MemoriLayoutTagged = DerivedLayout<MemoriLayout>

type DeriveKeys<T> = T extends unknown ? keyof T : never

declare global {
	namespace Memori {
		type Widget = MemoriWidget
		type Layout = MemoriLayoutTagged
		type LayoutSlot = MemoriLayoutSlot
		type WidgetKind = DeriveKeys<MemoriWidgetKind>
		type WidgetId = MemoriWidgetId

		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

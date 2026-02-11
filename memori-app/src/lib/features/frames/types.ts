export type WidgetId = string

export type MemoriLayout =
	| { type: 'full'; widget: WidgetId }
	| { type: 'v_split'; left: WidgetId; right: WidgetId }
	| { type: 'h_split'; top: WidgetId; bottom: WidgetId }
	| {
			type: 'v_split_with_right_h_split'
			left: WidgetId
			rightTop: WidgetId
			rightBottom: WidgetId
	  }
	| {
			type: 'h_split_with_top_v_split'
			topLeft: WidgetId
			topRight: WidgetId
			bottom: WidgetId
	  }
	| {
			type: 'v_split_with_left_h_split'
			leftTop: WidgetId
			leftBottom: WidgetId
			right: WidgetId
	  }
	| {
			type: 'h_split_with_bottom_v_split'
			top: WidgetId
			bottomLeft: WidgetId
			bottomRight: WidgetId
	  }
	| {
			type: 'fourths'
			topLeft: WidgetId
			topRight: WidgetId
			bottomLeft: WidgetId
			bottomRight: WidgetId
	  }

export type WidgetKind = 'clock' | 'weather' | 'todo' | 'calendar' | 'custom'

export interface WidgetInstance<TConfig = unknown> {
	id: WidgetId
	kind: WidgetKind
	title?: string
	config: TConfig
}

export interface DashboardConfig {
	schemaVersion: 1
	layout: MemoriLayout
	widgets: Record<WidgetId, WidgetInstance>
}

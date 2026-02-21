/* example entry from Memori.Layout type:
{
  variant: "VSplit"
  widget: {
    left: MemoriWidgetId
    right: MemoriWidgetId
  }
}
*/

type LayoutVariant = Memori.Layout['variant']

type WidgetPayload<V extends LayoutVariant> = Extract<
	Memori.Layout,
	{ variant: V }
>['widgets']

type SlotName<V extends LayoutVariant> = Extract<keyof WidgetPayload<V>, string>

// layout slot (e.g. HSplit.top valid, HSplit.bottomRight invalid)
export type SlotPosition = {
	[V in LayoutVariant]: `${V}.${SlotName<V>}`
}[LayoutVariant]

export type SlotId = SlotPosition | 'widget-pool'

export type SlotMeta = {
	id: SlotPosition
	classes: string
}

export type Containers = Record<SlotId, Memori.Widget[]>

// SlotId as shared primary key
export type DndState = {
	containerClasses: Record<LayoutVariant, string>
	slotsMeta: SlotMeta[] // static
	containers: Containers // dynamic state
}

type LayoutTemplate = {
	[V in LayoutVariant]: {
		// css classes for structuring
		container: string
		slots: {
			[K in SlotName<V>]: { classes: string }
		}
	}
}

export const LAYOUT_TEMPLATES: LayoutTemplate = {
	Full: {
		container: 'grid-cols-1 grid-rows-1',
		slots: {
			center: { classes: '' },
		},
	},
	VSplit: {
		container: 'grid-cols-2 grid-rows-1',
		slots: {
			left: { classes: 'col-start-1' },
			right: { classes: 'col-start-2' },
		},
	},
	HSplit: {
		container: 'grid-cols-1 grid-rows-2',
		slots: {
			top: { classes: 'row-start-1' },
			bottom: { classes: 'row-start-2' },
		},
	},
	VSplitWithRightHSplit: {
		container: 'grid-cols-2 grid-rows-2',
		slots: {
			left: { classes: 'col-start-1 row-start-1 row-end-3' },
			rightTop: { classes: 'col-start-2 row-start-1' },
			rightBottom: { classes: 'col-start-2 row-start-2' },
		},
	},
	HSplitWithTopVSplit: {
		container: 'grid-cols-2 grid-rows-2',
		slots: {
			topLeft: { classes: 'col-start-1 row-start-1' },
			topRight: { classes: 'col-start-2 row-start-1' },
			bottom: { classes: 'col-start-1 col-end-3 row-start-2' },
		},
	},
	VSplitWithLeftHSplit: {
		container: 'grid-cols-2 grid-rows-2',
		slots: {
			leftTop: { classes: 'col-start-1 row-start-1' },
			leftBottom: { classes: 'col-start-1 row-start-2' },
			right: { classes: 'col-start-2 row-start-1 row-end-3' },
		},
	},
	HSplitWithBottomVSplit: {
		container: 'grid-cols-2 grid-rows-2',
		slots: {
			top: { classes: 'col-start-1 col-end-3 row-start-1' },
			bottomLeft: { classes: 'col-start-1 row-start-2' },
			bottomRight: { classes: 'col-start-2 row-start-2' },
		},
	},
	Fourths: {
		container: 'grid-cols-2 grid-rows-2',
		slots: {
			topLeft: { classes: 'col-start-1 row-start-1' },
			topRight: { classes: 'col-start-2 row-start-1' },
			bottomLeft: { classes: 'col-start-1 row-start-2' },
			bottomRight: { classes: 'col-start-2 row-start-2' },
		},
	},
}

// {
//   slotMetas: [{ id: "1", position: "Full.center", classes: "..." }, { id: "2", position: "VSplit.left", classes: "..." }, { id: "3", position: "VSplit.right", classes: "..." }, ...]
//   containers: { "1": [name_widget], "2": [clock_widget], "3": [name_widget]}
// }

const PRECOMPUTED_LAYOUT = (
	Object.entries(LAYOUT_TEMPLATES) as [
		LayoutVariant,
		(typeof LAYOUT_TEMPLATES)[LayoutVariant],
	][]
).reduce(
	(acc, [variant, tpl]) => {
		acc.containerClasses[variant] = tpl.container

		for (const [slotName, cfg] of Object.entries(tpl.slots)) {
			acc.allSlotsMeta.push({
				id: `${variant}.${slotName}` as SlotPosition,
				classes: cfg.classes,
			})
		}
		return acc
	},
	{
		containerClasses: {} as Record<LayoutVariant, string>,
		allSlotsMeta: [] as SlotMeta[],
	}
)

export const createLayoutState = (pool: Memori.Widget[]): DndState => {
	const containers = { 'widget-pool': pool } as Containers

	for (const { id } of PRECOMPUTED_LAYOUT.allSlotsMeta) {
		containers[id] = []
	}

	return {
		containerClasses: PRECOMPUTED_LAYOUT.containerClasses,
		slotsMeta: PRECOMPUTED_LAYOUT.allSlotsMeta,
		containers,
	}
}

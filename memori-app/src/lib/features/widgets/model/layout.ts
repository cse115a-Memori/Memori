export type LayoutVariant = Memori.Layout['variant']

type WidgetPayload<V extends LayoutVariant> = Extract<
	Memori.Layout,
	{ variant: V }
>['widgets']

type SlotName<V extends LayoutVariant> = V extends LayoutVariant
	? Extract<keyof WidgetPayload<V>, string>
	: never

export type LayoutSlotPos = {
	[V in LayoutVariant]: `${V}.${SlotName<V>}`
}[LayoutVariant]

type LayoutTemplate = {
	[V in LayoutVariant]: {
		container: string
		slots: readonly { name: SlotName<V>; classes: string }[]
	}
}

export const LAYOUT_TEMPLATES: LayoutTemplate = {
	Full: {
		container: 'grid-cols-1 grid-rows-1',
		slots: [{ name: 'center', classes: '' }],
	},
	VSplit: {
		container: 'grid-cols-2 grid-rows-1',
		slots: [
			{ name: 'left', classes: 'col-start-1' },
			{ name: 'right', classes: 'col-start-2' },
		],
	},
	HSplit: {
		container: 'grid-cols-1 grid-rows-2',
		slots: [
			{ name: 'top', classes: 'row-start-1' },
			{ name: 'bottom', classes: 'row-start-2' },
		],
	},
	VSplitWithRightHSplit: {
		container: 'grid-cols-2 grid-rows-2',
		slots: [
			{ name: 'left', classes: 'col-start-1 row-start-1 row-end-3' },
			{ name: 'rightTop', classes: 'col-start-2 row-start-1' },
			{ name: 'rightBottom', classes: 'col-start-2 row-start-2' },
		],
	},
	HSplitWithTopVSplit: {
		container: 'grid-cols-2 grid-rows-2',
		slots: [
			{ name: 'topLeft', classes: 'col-start-1 row-start-1' },
			{ name: 'topRight', classes: 'col-start-2 row-start-1' },
			{ name: 'bottom', classes: 'col-start-1 col-end-3 row-start-2' },
		],
	},
	VSplitWithLeftHSplit: {
		container: 'grid-cols-2 grid-rows-2',
		slots: [
			{ name: 'leftTop', classes: 'col-start-1 row-start-1' },
			{ name: 'leftBottom', classes: 'col-start-1 row-start-2' },
			{ name: 'right', classes: 'col-start-2 row-start-1 row-end-3' },
		],
	},
	HSplitWithBottomVSplit: {
		container: 'grid-cols-2 grid-rows-2',
		slots: [
			{ name: 'top', classes: 'col-start-1 col-end-3 row-start-1' },
			{ name: 'bottomLeft', classes: 'col-start-1 row-start-2' },
			{ name: 'bottomRight', classes: 'col-start-2 row-start-2' },
		],
	},
	Fourths: {
		container: 'grid-cols-2 grid-rows-2',
		slots: [
			{ name: 'topLeft', classes: 'col-start-1 row-start-1' },
			{ name: 'topRight', classes: 'col-start-2 row-start-1' },
			{ name: 'bottomLeft', classes: 'col-start-1 row-start-2' },
			{ name: 'bottomRight', classes: 'col-start-2 row-start-2' },
		],
	},
}

export const LAYOUT_VARIANTS = Object.keys(LAYOUT_TEMPLATES) as Array<
	keyof typeof LAYOUT_TEMPLATES
>

export const getLayoutSlotCount = (variant: LayoutVariant): number =>
	LAYOUT_TEMPLATES[variant].slots.length

export const getSlotClassByIndex = (variant: LayoutVariant, index: number): string =>
	LAYOUT_TEMPLATES[variant].slots[index]?.classes ?? ''

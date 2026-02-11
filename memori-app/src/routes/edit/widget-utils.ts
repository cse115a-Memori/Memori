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

// layout slot (e.g. HSplit.top valid, HSplit.bottom_right invalid)
type LayoutSlotId = {
  [V in LayoutVariant]: `${V}.${SlotName<V>}`
}[LayoutVariant]

export type LayoutSlot = {
  id: string
  position: LayoutSlotId
  widget: Memori.Widget | null
}

type LayoutTemplate = {
  [V in LayoutVariant]: {
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
      right_top: { classes: 'col-start-2 row-start-1' },
      right_bottom: { classes: 'col-start-2 row-start-2' },
    },
  },
  HSplitWithTopVSplit: {
    container: 'grid-cols-2 grid-rows-2',
    slots: {
      top_left: { classes: 'col-start-1 row-start-1' },
      top_right: { classes: 'col-start-2 row-start-1' },
      bottom: { classes: 'col-start-1 col-end-3 row-start-2' },
    },
  },
  VSplitWithLeftHSplit: {
    container: 'grid-cols-2 grid-rows-2',
    slots: {
      left_top: { classes: 'col-start-1 row-start-1' },
      left_bottom: { classes: 'col-start-1 row-start-2' },
      right: { classes: 'col-start-2 row-start-1 row-end-3' },
    },
  },
  HSplitWithBottomVSplit: {
    container: 'grid-cols-2 grid-rows-2',
    slots: {
      top: { classes: 'col-start-1 col-end-3 row-start-1' },
      bottom_left: { classes: 'col-start-1 row-start-2' },
      bottom_right: { classes: 'col-start-2 row-start-2' },
    },
  },
  Fourths: {
    container: 'grid-cols-2 grid-rows-2',
    slots: {
      top_left: { classes: 'col-start-1 row-start-1' },
      top_right: { classes: 'col-start-2 row-start-1' },
      bottom_left: { classes: 'col-start-1 row-start-2' },
      bottom_right: { classes: 'col-start-2 row-start-2' },
    },
  },
}

export const slotsFor = (variant: LayoutVariant) => {
  return Object.entries(LAYOUT_TEMPLATES[variant].slots).map(([slot, cfg]) => ({
    position: `${variant}.${slot}`,
    classes: cfg.classes,
  }))
}

export type WidgetCtx = {
  widgetPool: Memori.Widget[]
  widgetFrame: LayoutSlot[]
}

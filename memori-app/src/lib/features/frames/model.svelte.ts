type DragPayload =
  | { source: "palette"; widgetId: string }
  | { source: "slot"; widgetId: string; from: SlotKey };

type DropMode = "replace" | "swap" | "reject_if_occupied";

function canDrop(type: LayoutType, target: SlotKey): boolean {
  return REQUIRED_SLOTS[type].includes(target);
}

function applyDrop(
  type: LayoutType,
  assignments: Assignments,
  drag: DragPayload,
  target: SlotKey,
  mode: DropMode
): Assignments {
  if (!canDrop(type, target)) return assignments;

  const next = { ...assignments };
  const existing = next[target];

  if (drag.source === "slot" && drag.from === target) return next;

  if (!existing) {
    if (drag.source === "slot") delete next[drag.from];
    next[target] = drag.widgetId;
    return next;
  }

  if (mode === "reject_if_occupied") return next;

  if (mode === "replace") {
    if (drag.source === "slot") delete next[drag.from];
    next[target] = drag.widgetId;
    return next;
  }

  // swap
  if (drag.source === "slot") {
    next[drag.from] = existing;
    next[target] = drag.widgetId;
  } else {
    // palette can't swap cleanly; treat as replace
    next[target] = drag.widgetId;
  }
  return next;
}

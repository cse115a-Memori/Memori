<script lang="ts" module>
  import { useSortable } from '@dnd-kit-svelte/svelte/sortable'
  import { CollisionPriority } from '@dnd-kit/abstract'
  import type { Snippet } from 'svelte'

  interface ColumnItemProps {
    children: Snippet<[isDragging: boolean]>
    data: { title: string; desc: string }
  }
</script>

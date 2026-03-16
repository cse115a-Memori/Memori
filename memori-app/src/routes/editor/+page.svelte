<script lang="ts">
  import WidgetEditor from '@/components/layout/WidgetEditor.svelte'
</script>

<main class="mx-auto space-y-6 py-8">
  <header class="space-y-2">
    <h1 class="text-2xl font-semibold tracking-tight">Editor</h1>
    <p class="text-sm text-muted-foreground">
      Arrange and preview the widgets that will be sent to your Memori device.
    </p>
  </header>

  <WidgetEditor />
</main>

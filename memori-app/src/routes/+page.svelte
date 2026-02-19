<script lang="ts">
  import { Button } from '$lib/components/ui/button'
</script>

<main class="space-y-6 py-8">
  <header class="space-y-2">
    <h1 class="text-2xl font-semibold tracking-tight">Memori App</h1>
    <p class="text-sm text-muted-foreground">
      Use the routes below for auth, device controls, drag-and-drop, and
      location.
    </p>
  </header>

  <div class="flex flex-wrap gap-3">
    <Button variant="outline" href="/login">Login</Button>
    <Button variant="outline" href="/device">Device</Button>
    <Button variant="outline" href="/test">Test</Button>
    <Button variant="outline" href="/location">Location</Button>
  </div>
</main>

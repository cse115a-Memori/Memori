<script lang="ts">
  import {
    checkPermissions,
    getCurrentPosition,
    type PermissionStatus,
    type Position,
    requestPermissions,
    watchPosition,
  } from '@tauri-apps/plugin-geolocation'
  import { Button } from '@/components/ui/button'

  let position = $state<Position | null>()
  let permission = $state<PermissionStatus['location']>('prompt')

  const getPos = async () => {
    let permissions = await checkPermissions()
    if (
      permissions.location === 'prompt' ||
      permissions.location === 'prompt-with-rationale'
    ) {
      permissions = await requestPermissions(['location'])
    }

    if (permissions.location === 'granted') {
      permission = 'granted'
      const pos = await getCurrentPosition()

      await watchPosition(
        { enableHighAccuracy: true, timeout: 10000, maximumAge: 0 },
        (pos) => {
          position = pos
        }
      )
    }
  }

  const checkPermission = async () => {
    let permissions = await checkPermissions()
    permission = permissions.location
    if (permission === 'granted') {
      await getPos()
    }

    return permissions
  }
</script>

<div>
  {#await checkPermission()}
    loading...
  {:then permissionStatus}
    {#if permission === 'prompt'}
      status: {permissionStatus.location}
    {:else}
      status: {permission}
    {/if}
    <div>
      {#if permission === 'granted'}
        <div>
          lat: {position?.coords.latitude}
        </div>
        <div>
          long: {position?.coords.longitude}
        </div>
      {:else if permission === 'denied'}
        You have to enable manually in settings.
      {:else}
        <Button onclick={getPos}>Enable Location</Button>
      {/if}
    </div>
  {:catch err}
    {err}
  {/await}
</div>

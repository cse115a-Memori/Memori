<script lang="ts">
  import { T } from '@threlte/core'
  import { useTexture } from '@threlte/extras'
  import { onMount } from 'svelte'
  import { TextGeometry } from 'three/addons/geometries/TextGeometry.js'
  import { FontLoader } from 'three/examples/jsm/loaders/FontLoader.js'

  let { weatherText = 'Hello World', y, z } = $props()

  let size = 1
  let depth = 0.18

  let textGeometry: TextGeometry | undefined = $state()
  let loaded = $state(false)

  const matcapTexture = useTexture('textures/matcaps/6.png')

  onMount(() => {
    const loader = new FontLoader()
    loader.load('fonts/helvetiker_regular.typeface.json', (font) => {
      textGeometry = new TextGeometry(weatherText, {
        font: font,
        size: size,
        depth: depth,
        curveSegments: 4,
        bevelEnabled: true,
        bevelThickness: 0.002,
        bevelSize: 0.04,
        bevelOffset: 0,
        bevelSegments: 5,
      })

      textGeometry.center()

      loaded = true
    })
  })
</script>

{#if loaded}
  {#await matcapTexture then matcap}
    <T.Mesh
      geometry={textGeometry}
      position.x={0}
      position.y={y}
      position.z={z}
    >
      <T.MeshMatcapMaterial {matcap} />
    </T.Mesh>
  {/await}
{/if}

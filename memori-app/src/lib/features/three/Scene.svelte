<script lang="ts">
  import { T } from '@threlte/core'
  import { interactivity, OrbitControls, SVG } from '@threlte/extras'
  import { onMount } from 'svelte'
  import { Spring } from 'svelte/motion'
  import DText from './3DText.svelte'

  onMount(() => {})

  const { target } = interactivity()
  target.set(document.getElementById('int-target') ?? undefined)

  const pos = new Spring({ x: 2, z: 1 })
  const setRandomPos = () => {
    pos.set({
      x: (Math.random() - 0.5) * 5,
      z: (Math.random() - 0.5) * 5,
    })
  }
</script>

<SVG
  src="/weather/sun-medium.svg"
  scale={0.1}
  position.x={-8}
  position.y={6}
  position.z={1}
/>

<DText weatherText={'Santa Cruz: 19°C'} y={5} z={1} />

<T.PerspectiveCamera makeDefault position={[0, 5, 35]} fov={25}>
  <OrbitControls enableDamping autoRotateSpeed={0.35} target.y={1.2} />
</T.PerspectiveCamera>

<T.DirectionalLight intensity={0.5} position.x={5} position.y={10} />
<T.AmbientLight intensity={0.35} />

<T.Mesh position={[0, -2.25, 0]} rotation.x={-Math.PI / 2}>
  <T.CircleGeometry args={[7, 64]} />
  <T.MeshStandardMaterial color="#f1e8dd" metalness={0.1} roughness={0.8} />
</T.Mesh>

<!--
	<T.Mesh
		position={[1.2, 1.5, 0.75]}
		rotation.x={5}
		rotation.y={71}
		on:click={setRandomPos}
	>
		<T.TorusKnotGeometry args={[0.5, 0.15, 100, 12, 2, 3]} />
		<T.MeshStandardMaterial color="#F85122" />
	</T.Mesh>
-->

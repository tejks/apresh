<script lang="ts">
	import { Marker, Popup } from 'svelte-maplibre';
	import { defaultLocation, type Coords } from '../lib/common';
	import Button from './Button.svelte';

	let {
		picked,
		name
	}: {
		name: string;
		picked: (coords: Coords) => void;
	} = $props();

	let location = $state(defaultLocation);
</script>

<Marker bind:lngLat={location} draggable>
	<div class="pin active bounce-a"></div>
	<Popup open offset={[-5, -10]}>
		<div class="text-sm font-bold">{name}</div>
	</Popup>
</Marker>

<header class="fixed top-0 z-50 w-full bg-transparent">
	<div class="flex items-center px-8 py-6">
		<div class="flex justify-start space-x-5">
			<Button onClick={() => picked(location)}>Confirm location</Button>
		</div>
	</div>
</header>

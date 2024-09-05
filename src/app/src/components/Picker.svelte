<script lang="ts">
	import { Marker, Popup } from 'svelte-maplibre';
	import { defaultLocation, type Coords } from '../lib/common';
	import Button from './button.svelte';
	import { wallet } from '$lib/wallet';

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
	<div class="pin active"></div>
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

<style lang="scss">
	.pin {
		width: 30px;
		height: 30px;
		border-radius: 50% 50% 50% 0;

		position: absolute;
		transform: rotate(-45deg);
		left: 50%;
		top: 50%;
		margin: -20px 0 0 -20px;

		&.active {
			background: var(--accent);
		}

		&.inactive {
			background: var(--secondary-200);
		}

		&.destination {
			background: #f00;
		}

		&.carrier.inactive {
			background: var(--primary-200);
		}

		&.carrier.active {
			background: var(--primary-600);
		}

		&:after {
			content: '';
			width: 14px;
			height: 14px;
			margin: 8px 0 0 8px;
			background: #e6e6e6;
			position: absolute;
			border-radius: 50%;
		}
	}
</style>

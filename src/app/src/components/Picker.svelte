<script lang="ts">
	import { Marker, Popup } from 'svelte-maplibre';
	import { defaultLocation, type Coords } from '../lib/common';

	let {
		picked = $bindable(),
		name
	}: {
		picked: Coords;
		name: string;
	} = $props();
</script>

<Marker bind:lngLat={picked} draggable>
	<div class="pin active"></div>
	<Popup open offset={[-5, -10]}>
		<div class="text-sm font-bold">{name}</div>
	</Popup>
</Marker>

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

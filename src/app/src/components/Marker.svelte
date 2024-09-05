<script lang="ts">
	import { Marker, Popup } from 'svelte-maplibre';
	import { defaultLocation, type Coords } from '../lib/common';
	import Button from './Button.svelte';

	let {
		location,
		name,
		onClick
	}: {
		name: string;
		location: Coords;
		onClick: () => void;
	} = $props();
</script>

<Marker bind:lngLat={location} on:click={onClick}>
	<div class="pin active bounce-a"></div>
	<Popup open offset={[-5, -10]}>
		<div onclick={onClick} class="text-sm font-bold">{name}</div>
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
			background: linear-gradient(
				90deg,
				rgb(209, 121, 21) 0%,
				rgb(247, 147, 66) 35%,
				rgba(0, 212, 255, 1) 100%
			);
		}

		&.inactive {
			background: rgb(131, 96, 67);
		}

		&.destination {
			background: #f00;
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

	.bounce-a {
		animation-name: bounce-a;
		animation-fill-mode: both;
		animation-duration: 1s;
	}

	@keyframes pulsate {
		0% {
			transform: scale(0.1, 0.1);
			opacity: 0;
		}

		50% {
			opacity: 1;
		}

		100% {
			transform: scale(1.2, 1.2);
			opacity: 0;
		}
	}

	@keyframes bounce-a {
		0% {
			opacity: 0;
			transform: translateY(-2000px) rotate(-45deg);
		}

		60% {
			opacity: 1;
			transform: translateY(30px) rotate(-45deg);
		}

		80% {
			transform: translateY(-10px) rotate(-45deg);
		}

		100% {
			transform: translateY(0) rotate(-45deg);
		}
	}
</style>

<script lang="ts">
	import { defaultLocation, type Coords } from '$lib/common';
	import { wallet } from '$lib/wallet';
	import Picker from '../components/Picker.svelte';

	let greeting = $state('');
	let name = $state('');

	async function onSubmit(event: MouseEvent) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		greeting = await $wallet.actor.greet(name);
	}

	function onLocationChange(coords: Coords) {
		console.log(coords);
	}

	let showModal = $state(true);
</script>

{#if showModal}
	<Picker name="From" picked={onLocationChange} />
{/if}

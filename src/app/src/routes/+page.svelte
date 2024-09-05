<script lang="ts">
	import { type Coords } from '$lib/common';
	import { wallet } from '$lib/wallet.svelte';
	import Picker from '../components/Picker.svelte';

	let greeting = $state('');
	let name = $state('');

	async function onSubmit(event: MouseEvent) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;
	}

	function onLocationChange(coords: Coords) {
		console.log(coords);
	}

	let showModal = $state(true);
</script>

{#if showModal}
	<Picker name="From" picked={onLocationChange} />
{/if}

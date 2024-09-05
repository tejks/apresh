<script lang="ts">
	import { defaultLocation } from '$lib/common';
	import { wallet } from '$lib/wallet';
	import Picker from '../components/Picker.svelte';

	let greeting = $state('');
	let name = $state('');

	async function onSubmit(event: MouseEvent) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		greeting = await $wallet.actor.greet(name);
	}

	let picked = $state(defaultLocation);
</script>

<Picker bind:picked name="From" />

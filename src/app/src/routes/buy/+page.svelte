<script lang="ts">
	import { wallet } from '$lib/wallet';
	import { onMount } from 'svelte';

	let shipments = $derived(async () => {});

	onMount(async () => {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return [];

		let shipments = await $wallet.actor.listPendingShipments();

		const balance = await wallet.balance();

		console.log(balance);
		console.log('idenity', $wallet.identity.getPrincipal().toString());
		await wallet.approve(300n);

		const error = await $wallet.actor.createShipment('Ja', {
			destination: { lat: 0, lng: 0, street: '' },

			value: 100n,
			source: { lat: 0, lng: 0, street: '' },
			size_category: {
				Parcel: {
					max_depth: 0n,
					max_height: 0n,
					max_width: 0n
				}
			},
			price: 200n
		});

		console.log(error);

		console.log(shipments);
	});

	// console.log();
</script>

<!-- {#await shipments()}
	Fetching shipments...
{:then value}
	{#each value as shipment}
		<div>{shipment}</div>
	{/each}
{/await} -->

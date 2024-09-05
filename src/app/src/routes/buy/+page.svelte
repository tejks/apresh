<script lang="ts">
	import { wallet } from '$lib/wallet';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import { invalidateAll } from '$app/navigation';

	const {
		data
	}: {
		data: PageData;
	} = $props();

	onMount(async () => {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return [];

		let shipments = await $wallet.actor.listPendingShipments();

		const balance = await wallet.balance();

		console.log(balance);
		console.log('idenity', $wallet.identity.getPrincipal().toString());
		await wallet.approve(balance);

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

	invalidateAll();
</script>

{#each data.shipments as shipment}
	<div>{shipment.id}</div>
{/each}

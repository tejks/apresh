<script lang="ts">
	import { wallet } from '$lib/wallet';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import { invalidateAll } from '$app/navigation';
	import Modal from '../../components/Modal.svelte';
	import Marker from '../../components/Marker.svelte';
	import type { Shipment } from '../../../../declarations/contract/contract.did';
	import Button from '../../components/Button.svelte';

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

		// const error = await $wallet.actor.createShipment('Ja', {
		// 	destination: { lat: 0, lng: 0, street: '' },

		// 	value: 100n,
		// 	source: { lat: 0, lng: 0, street: '' },
		// 	size_category: {
		// 		Parcel: {
		// 			max_depth: 0n,
		// 			max_height: 0n,
		// 			max_width: 0n
		// 		}
		// 	},
		// 	price: 200n
		// });

		// console.log(error);

		console.log(shipments);
	});

	invalidateAll();

	function selectShipment(id: bigint) {
		console.log(id);
		selected = data.shipments.find((shipment) => shipment.id === id) ?? undefined;
		console.log(selected);
		showModal = true;
	}

	let selected = $state<Shipment | null>(null);
	let parcel = $derived(selected ? Object.keys(selected?.info.size_category)[0] : null);

	let showModal = $state(false);

	async function buy(shipment: Shipment) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		wallet;
		wallet.approve(shipment.info.price);

		// const error = await $wallet.actor.buyShipment(shipment.id);

		// if (error) {
		// 	console.error(error);
		// } else {
		// 	console.log('bought');
		// }
	}
</script>

{#each data.shipments as { id, info }}
	<Marker onClick={() => selectShipment(id)} location={info.source} name={id}></Marker>
{/each}

<Modal bind:showModal>
	<h1>
		Buy shipment:
		{selected?.id}
	</h1>
	<br />

	<ul>
		{#if selected}
			<li>From: {selected.info.source.street}</li>
			<li>To: {selected.info.destination.street}</li>
			<li>Price: {selected.info.price}</li>
			<li>Size: {parcel}</li>
		{/if}
	</ul>

	<Button onClick={() => buy(selected!)}>Buy</Button>
</Modal>

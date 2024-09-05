<script lang="ts">
	import { wallet } from '$lib/wallet.svelte';
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

		const fee = await wallet.getTransferFee();
		wallet.approve(shipment.info.value + fee);

		const error = await $wallet.actor.buyShipment('Jacek', shipment.id);
		console.log(error);

		await invalidateAll();

		selected = null;
		showModal = false;
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

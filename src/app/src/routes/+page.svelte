<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { type Coords } from '$lib/common';
	import { wallet } from '$lib/wallet.svelte';
	import type { Shipment } from '../../../declarations/contract/contract.did';
	import Modal from '../components/Modal.svelte';
	import Picker from '../components/Picker.svelte';
	import ShipmentInfo from '../components/ShipmentInfo.svelte';
	import type { PageData } from './$types';
	import Button from '../components/Button.svelte';
	import Marker from '../components/Marker.svelte';
	import { onMount } from 'svelte';

	onMount(async () => {
		await wallet.connect();

		await invalidateAll();
	});

	const {
		data
	}: {
		data: PageData;
	} = $props();

	let greeting = $state('');
	let name = $state('');

	function onLocationChange(coords: Coords) {
		console.log(coords);
	}

	let showModal = $state(true);

	function selectShipment(id: bigint) {
		selected =
			[...data.shipments, ...data.created].find((shipment) => shipment.id === id) ?? undefined;
		showBuyModal = true;
	}

	async function buy(shipment: Shipment) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		const fee = await wallet.getTransferFee();
		wallet.approve(shipment.info.value + fee);

		const error = await $wallet.actor.buyShipment('Jacek', shipment.id);
		console.log(error);

		await invalidateAll();

		selected = null;
		showBuyModal = false;
	}

	async function settle(shipment: Shipment) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		const fee = await wallet.getTransferFee();
		wallet.approve(shipment.info.value + fee);

		const error = await $wallet.actor.finalizeShipment(shipment.id);
		console.log(error);

		await invalidateAll();

		selected = null;
		showBuyModal = false;
	}

	let showBuyModal = $state(false);
	let selected = $state<Shipment | null>(null);
</script>

{#if showModal}
	<Picker name="From" picked={onLocationChange} />
{/if}

{#if data.created.length == 0}
	{#each data.shipments as { id, info }}
		<Marker onClick={() => selectShipment(id)} location={info.source} name={id}></Marker>
	{/each}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />
		{/if}

		<Button onClick={() => buy(selected!)}>Buy</Button>
	</Modal>
{:else}
	{#each data.created as { id, info }}
		<Marker onClick={() => selectShipment(id)} location={info.destination} name={id}></Marker>
	{/each}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />
		{/if}

		<Button onClick={() => settle(selected!)}>Settle</Button>
	</Modal>
{/if}

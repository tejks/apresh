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
	import CreateShipmentForm from '../components/CreateShipment.svelte';
	import { Plus } from 'lucide-svelte';

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
			[...data.shipments, ...data.created, ...data.carried].find(
				(shipment) => shipment.id === id
			) ?? undefined;
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

		const error = await $wallet.actor.finalizeShipment(shipment.id);
		console.log(error);

		await invalidateAll();

		selected = null;
		showBuyModal = false;
	}

	let showBuyModal = $state(false);
	let showAddModal = $state(false);
	let selected = $state<Shipment | null>(null);
</script>

{#if showModal}
	<Picker name="From" picked={onLocationChange} />
{/if}

<CreateShipmentForm showModal={showAddModal} onClose={() => (showAddModal = false)} />

{#if data.created.length > 0}
	{#each data.created as { id, info }}
		<Marker onClick={() => selectShipment(id)} location={info.destination} name={id}></Marker>
	{/each}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />
		{/if}

		<Button onClick={() => settle(selected!)}>Settle</Button>
	</Modal>

	<div class="absolute bottom-16 right-16 z-50">
		<CreateShipmentForm {showModal} onClose={() => (showModal = false)} />

		<div
			class="flex rounded-full mx-auto bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-0.5 shadow-lg transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
		>
			<button
				onclick={() => (showModal = true)}
				class="rounded-full w-20 h-20 bg-white flex justify-center items-center"
			>
				<Plus size={55} class="stroke-orange-400" />
			</button>
		</div>
	</div>
{:else if data.carried.length > 0}
	{#each data.carried as { id, info }}
		<Marker onClick={() => selectShipment(id)} location={info.destination} name={id}></Marker>
	{/each}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />
		{/if}

		<!-- <Button onClick={() => cancel(selected!)}>Settle</Button> -->
	</Modal>
{:else}
	{#each data.shipments as { id, info }}
		<Marker onClick={() => selectShipment(id)} location={info.source} name={id}></Marker>
	{/each}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />
		{/if}

		<Button onClick={() => buy(selected!)}>Buy</Button>
	</Modal>

	<div class="absolute bottom-16 right-16 z-50">
		<CreateShipmentForm {showModal} onClose={() => (showModal = false)} />

		<div
			class="flex rounded-full mx-auto bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-0.5 shadow-lg transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
		>
			<button
				onclick={() => (showModal = true)}
				class="rounded-full w-20 h-20 bg-white flex justify-center items-center"
			>
				<Plus size={55} class="stroke-orange-400" />
			</button>
		</div>
	</div>
{/if}

<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { wallet } from '$lib/wallet.svelte';
	import { Plus } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import type { Shipment } from '../../../declarations/contract/contract.did';
	import CreateShipmentForm from '../components/CreateShipment.svelte';
	import Marker from '../components/Marker.svelte';
	import Modal from '../components/Modal.svelte';
	import ShipmentInfo from '../components/ShipmentInfo.svelte';
	import type { PageData } from './$types';
	import { anonymousBackend } from '$lib/canisters';
	import { getLocalStorage } from '$lib/storage';
	import Input from '../components/Input.svelte';
	import TextInput from '../components/common/Inputs/TextInput.svelte';

	onMount(async () => {
		await wallet.connect();
		await invalidateAll();
	});

	const {
		data
	}: {
		data: PageData;
	} = $props();

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

		const errorMessage = await $wallet.actor.addEncryptedMessage(message, shipment.id);
		console.log(errorMessage);

		await invalidateAll();

		selected = null;
		showBuyModal = false;
	}

	async function settle(shipment: Shipment) {
		if (!$wallet.connected) await wallet.connect();
		if (!$wallet.connected) return;

		const error = await $wallet.actor.finalizeShipment(shipment.id, []);
		console.log(error);

		await invalidateAll();

		selected = null;
		showBuyModal = false;
	}

	let showBuyModal = $state(false);
	let showAddModal = $state(false);
	let message = $state('');
	let selected = $state<Shipment | null>(null);

	$effect(() => {
		console.log('data', data);
		if (!data.settleId || !data.settleSecret) return;

		console.log('here');
		anonymousBackend
			.finalizeShipment(BigInt(data.settleId), [data.settleSecret])
			.then((res) => console.log('endpoint res ', res));
	});
</script>

<CreateShipmentForm showModal={showAddModal} onClose={() => (showAddModal = false)} />

{#if data.created.length > 0}
	{#each data.created as { id, info }}
		<Marker onClick={() => selectShipment(id)} location={info.destination} name={id}></Marker>
	{/each}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />

			<a href="/?settleId={selected?.id}&settleSecret={getLocalStorage(selected!.id.toString())}">
				/?settleId={selected?.id}&settleSecret={getLocalStorage(selected!.id.toString())}
			</a>

			<button
				class="bg-gradient-to-r from-blue-500 to-rose-400 rounded-full px-7 py-2 w-1/2 mx-auto text-white text-base"
			>
				Share
			</button>

			<button
				class="bg-gradient-to-r from-blue-500 to-rose-400 rounded-full px-7 py-2 w-1/2 mx-auto text-white text-base"
				onclick={() => settle(selected!)}>Settle</button
			>
		{/if}
	</Modal>

	<div class="absolute bottom-16 right-16 z-50">
		<div
			class="flex rounded-full mx-auto bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-0.5 shadow-lg transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
		>
			<button
				onclick={() => {
					if (!$wallet.connected) wallet.connect();
					showAddModal = true;
				}}
				class="rounded-full w-20 h-20 bg-white flex justify-center items-center"
			>
				<Plus size={55} class="stroke-orange-400" />
			</button>
		</div>
	</div>
{:else if data.carried.length > 0}
	{#if !showAddModal}
		{#each data.carried as { id, info }}
			<Marker onClick={() => selectShipment(id)} location={info.destination} name={id}></Marker>
		{/each}
	{/if}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />
		{/if}
	</Modal>
{:else}
	{#if !showAddModal}
		{#each data.shipments as { id, info }}
			<Marker onClick={() => selectShipment(id)} location={info.source} name={id}></Marker>
		{/each}
	{/if}

	<Modal bind:showModal={showBuyModal} onClose={() => (showBuyModal = false)}>
		{#if selected}
			<ShipmentInfo shipment={selected} />
		{/if}

		<TextInput id ="Message" label="Message" name="Message"  bind:value={message}/>
 
		<button
			class="bg-gradient-to-r from-blue-500 to-rose-400 rounded-full px-7 py-2 w-1/2 mx-auto text-white text-base"
			onclick={() => buy(selected!)}>Buy</button
		>
	</Modal>

	<div class="absolute bottom-16 right-16 z-50">
		<div
			class="flex rounded-full mx-auto bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-0.5 shadow-lg transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
		>
			<button
				onclick={() => {
					if (!$wallet.connected) wallet.connect();
					showAddModal = true;
				}}
				class="rounded-full w-20 h-20 bg-white flex justify-center items-center"
			>
				<Plus size={55} class="stroke-orange-400" />
			</button>
		</div>
	</div>
{/if}

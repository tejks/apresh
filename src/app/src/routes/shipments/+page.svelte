<script lang="ts">
	import { anonymousBackend } from '$lib/canisters';
	import { Plus } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import type { Shipment } from '../../../../declarations/contract/contract.did';
	import CreateShipmentForm from '../../components/CreateShipment.svelte';
	import ShipmantLocations from '../../components/ShipmantLocations.svelte';

	let shipments: Shipment[] = $state([]);
	let showModal = $state(false);

	onMount(async () => {
		shipments = await anonymousBackend.listPendingShipments();
		console.log(shipments);
	});
</script>

<div class="absolute bottom-16 right-16 z-50">
	<CreateShipmentForm {showModal} onClose={() => (showModal = false)} />

	<ShipmantLocations {shipments} />

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

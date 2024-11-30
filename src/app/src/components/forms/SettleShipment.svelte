<script lang="ts">
	import { getLocalStorage } from '$lib/storage';
	import QrCodeDisplay from '../QrCodeDisplay.svelte';
	import ShipmentInfo from '../ShipmentInfo.svelte';

	let { selected, onSettle } = $props<{
		selected: any;
		onSettle: (selected: any) => void;
	}>();

	let message = '';
</script>

<div class="flex justify-between w-full mx-5">
	<div class="flex flex-col">
		<ShipmentInfo shipment={selected} />

		<button
			class="bg-gradient-to-r from-blue-500 to-rose-400 rounded-full px-7 py-2 w-1/2 mx-auto text-white text-base transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
			onclick={() => onSettle(selected!)}
		>
			Settle
		</button>
	</div>
	<div class="flex items-center text-lg">OR</div>
	<div class="flex items-center">
		{#if selected}
			<QrCodeDisplay
				settleId={selected.id}
				settleSecret={getLocalStorage(selected.id.toString())}
			/>
		{/if}
	</div>
</div>

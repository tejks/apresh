<script lang="ts">
	import type { Shipment } from '../../../declarations/contract/contract.did';

	let { shipment }: { shipment: Shipment } = $props();
	let parcel = $derived(Object.keys(shipment.info.size_category)[0]);
</script>

<h1>
	Buy shipment:
	{shipment.id}
</h1>
<br />

<ul>
	{#if shipment}
		<li>From: {shipment.info.source.street}</li>
		<li>To: {shipment.info.destination.street}</li>
		<li>Price: {shipment.info.price}</li>
		<li>Size: {parcel}</li>
	{/if}
</ul>

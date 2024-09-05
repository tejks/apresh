import type { Shipment } from '../../../../declarations/contract/contract.did';
import { anonymousBackend } from '$lib/canisters';
import { stateWallet, wallet } from '$lib/wallet.svelte';

/** @type {import('./$types').PageLoad } */
export async function load({}): Promise<{
	shipments: Shipment[];
	registeredCarrier: boolean;
	registeredCustomer: boolean;
}> {
	const shipments = await anonymousBackend.listPendingShipments();

	let registeredCarrier = false;
	let registeredCustomer = false;

	if (stateWallet.actor) {
		const [car, cus] = await stateWallet.actor.roles();
		registeredCarrier = car;
		registeredCustomer = cus;
	}

	console.log('shipments', shipments);

	return {
		shipments,
		registeredCarrier,
		registeredCustomer
	};
}

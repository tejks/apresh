import type { Shipment } from '../../../declarations/contract/contract.did';
import { anonymousBackend } from '$lib/canisters';
import { stateWallet, wallet } from '$lib/wallet.svelte';

/** @type {import('./$types').PageLoad } */
export async function load({}): Promise<{
	shipments: Shipment[];
	registeredCarrier: boolean;
	registeredCustomer: boolean;
	carried: Shipment[];
	created: Shipment[];
}> {
	const shipments = await anonymousBackend.listPendingShipments();

	let registeredCarrier = false;
	let registeredCustomer = false;
	let carried: Shipment[] = [];
	let created: Shipment[] = [];

	if (stateWallet.actor) {
		console.log('Wallet connected');

		// const [car, cus] = await stateWallet.actor.roles();
		// registeredCarrier = car;
		// registeredCustomer = cus;

		// if (registeredCarrier) {
		console.log('Carrier registered');

		let [car, cus] = await stateWallet.actor.listUserShipments();
		carried = car;
		created = cus;
		// }
	}

	console.log('Shipments:', shipments);
	console.log('carried:', carried);
	console.log('created:', created);

	return {
		shipments,
		registeredCarrier,
		registeredCustomer,
		carried,
		created
	};
}

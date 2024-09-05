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
		const [car, cus] = await stateWallet.actor.roles();
		registeredCarrier = car;
		registeredCustomer = cus;
		if (registeredCarrier) {
			let [car, cus] = await stateWallet.actor.listUserShipments();
			carried = car;
			created = cus;
		}
	}

	return {
		shipments,
		registeredCarrier,
		registeredCustomer,
		carried,
		created
	};
}

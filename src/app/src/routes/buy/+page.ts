import type { Shipment } from '../../../../declarations/contract/contract.did';
import { anonymousBackend } from '$lib/canisters';

/** @type {import('./$types').PageLoad } */
export async function load({}): Promise<{
	shipments: Shipment[];
}> {
	const shipments = await anonymousBackend.listPendingShipments();

	return {
		shipments
	};
}

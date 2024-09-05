import type { ActorSubclass, Identity } from '@dfinity/agent';
import { writable } from 'svelte/store';
import type { _SERVICE } from '../../../declarations/contract/contract.did';
import { connect } from './canisters';

export const wallet = createWallet();

interface Wallet {
	connected: true;
	actor: ActorSubclass<_SERVICE>;
	identity: Identity;
}

function createWallet() {
	const { subscribe, set, update } = writable<{ connected: false } | Wallet>({
		connected: false
	});

	return {
		subscribe,
		connect: async () => {
			const { actor, identity } = await connect();
			console.log(identity.getPrincipal().toText())
			set({ connected: true, actor, identity });
		}
	};
}

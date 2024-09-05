import type { ActorSubclass, Identity } from '@dfinity/agent';
import { get, writable } from 'svelte/store';
import type { _SERVICE } from '../../../declarations/contract/contract.did';
import { canisterId } from '../../../declarations/contract';
import type { _SERVICE as _ICRC1_SERVICE } from '../../../declarations/icrc1_ledger_canister/icrc1_ledger_canister.did';
import { connect } from './canisters';
import { Principal } from '@dfinity/principal';

export const wallet = createWallet();

interface Wallet {
	connected: true;
	actor: ActorSubclass<_SERVICE>;
	tokenActor: ActorSubclass<_ICRC1_SERVICE>;
	identity: Identity;
}

function createWallet() {
	const { subscribe, set, update } = writable<{ connected: false } | Wallet>({
		connected: false
	});

	return {
		subscribe,
		connect: async () => {
			const { actor, tokenActor, identity } = await connect();
			console.log(identity.getPrincipal().toText());
			set({ connected: true, actor, tokenActor, identity });
		},
		approve: async (amount: bigint) => {
			const current = get(wallet);
			if (!current.connected) await wallet.connect();
			if (!current.connected) throw new Error('Not connected');

			const spender = Principal.fromText(canisterId);
			console.log(spender.toText());

			const approveResult = await current.tokenActor.icrc2_approve({
				fee: [],
				from_subaccount: [],
				memo: [],
				created_at_time: [],
				amount,
				expected_allowance: [],
				expires_at: [],
				spender: { owner: spender, subaccount: [] }
			});

			console.log(approveResult);
		},
		balance: async () => {
			const current = get(wallet);
			if (!current.connected) await wallet.connect();
			if (!current.connected) throw new Error('Not connected');

			const balance = await current.tokenActor.icrc1_balance_of({
				owner: current.identity.getPrincipal(),
				subaccount: []
			});
			return balance;
		}
	};
}

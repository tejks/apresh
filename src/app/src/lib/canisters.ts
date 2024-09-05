import { createActor, canisterId } from '../../../declarations/contract';
import { canisterId as identityCanisterId } from '../../../declarations/internet_identity';
import {
	canisterId as tokenCanisterId,
	createActor as createTokenActor
} from '../../../declarations/icrc1_ledger_canister';
import { AuthClient } from '@dfinity/auth-client';

const host = `http://localhost:4943`;

export const anonymousBackend = createActor(canisterId, { agentOptions: { host } });

export const connect = async () => {
	let authClient = await AuthClient.create();

	await new Promise((resolve) => {
		authClient.login({
			identityProvider: `http://${identityCanisterId}.localhost:4943/`, // 'https://identity.ic0.app'
			onSuccess: () => resolve(undefined)
		});
	});

	const identity = authClient.getIdentity();
	const actor = createActor(canisterId, {
		agentOptions: {
			identity,
			host
		}
	});
	const tokenActor = createTokenActor(tokenCanisterId, {
		agentOptions: {
			identity,
			host
		}
	});

	console.log('Connected to backend as', identity.getPrincipal().toText());

	return { actor, tokenActor, identity };
};

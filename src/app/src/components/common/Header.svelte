<script lang="ts">
	import { wallet } from '$lib/wallet';
	import Button from '../Button.svelte';
</script>

<header class="fixed top-0 z-50 w-full bg-transparent">
	<div class="flex items-center px-8 py-6">
		<div class="ml-auto flex justify-center space-x-5">
			{#if $wallet.connected && $wallet.identity}
				<Button onClick={() => {}}>
					Identity {$wallet.identity.getPrincipal().toText().substring(0, 6)}...
				</Button>
			{:else}
				<Button onClick={async () => await wallet.connect()}>Connect wallet</Button>
			{/if}
		</div>
	</div>
</header>

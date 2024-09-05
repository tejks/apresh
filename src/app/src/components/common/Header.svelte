<script lang="ts">
	import { wallet } from '$lib/wallet';
</script>

<header class="sticky top-0 z-50 w-full">
	<div class="flex items-center px-8 py-4">
		<div class="ml-auto flex justify-center space-x-5">
			{#if $wallet.connected && $wallet.identity}
				<button on:click={() => {}}>
					Identity: {$wallet.identity.getPrincipal().toText().substring(0, 6)}...
				</button>
			{:else}
				<button on:click={async () => await wallet.connect()}>Connect wallet</button>
			{/if}
		</div>
	</div>
</header>

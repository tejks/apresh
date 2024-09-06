<script lang="ts">
	import type { Snippet } from 'svelte';

	interface ButtonProps {
		onClick: () => void;
		children: Snippet;
	}

	const { children, onClick }: ButtonProps = $props();
</script>

<div
	class="flex rounded-full mx-auto bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-0.5 shadow-lg transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
>
	<button onclick={onClick} class="flex-1 bg-white px-7 py-2 rounded-full text-lg"
		>{@render children()}</button
	>
</div>

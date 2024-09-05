<script lang="ts">
	import { on } from 'svelte/events';

	let { onEnter, value = $bindable('') }: { onEnter: (text: string) => void; value: string } =
		$props();

	function onChange() {
		onEnter(value);
	}
</script>

<div
	class="flex rounded-full mx-auto bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-0.5 shadow-lg transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200"
>
	<input
		type="text"
		onchange={() => onEnter(value)}
		bind:value
		class="flex-1 bg-white px-7 py-2 rounded-full text-lg"
	/>
</div>

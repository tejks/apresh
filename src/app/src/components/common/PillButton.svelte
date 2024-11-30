<script lang="ts">
	let {
		onClick,
		text,
		className = ''
	} = $props<{
		onClick: () => void;
		text: string;
		className?: string;
	}>();
</script>

<button
	onclick={onClick}
	class="bg-gradient-to-r from-blue-500 to-rose-400 rounded-full px-7 py-2 text-white text-base transition ease-in-out hover:-translate-y-0.5 hover:scale-105 duration-200 {className}"
>
	{text}
</button>

<script lang="ts">
	import clsx from 'clsx';
	import type { Snippet } from 'svelte';

	interface IProps {
		showModal: boolean;
		onClose: () => void;
		cls?: string;
		children: Snippet;
		header: Snippet;
	}

	let { showModal = $bindable(), onClose, cls, children, header }: IProps = $props();

	let dialog: HTMLDialogElement;

	$effect(() => {
		if (dialog && showModal) dialog.showModal();
		if (dialog && !showModal) dialog.close();
	});
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<dialog
	bind:this={dialog}
	class={clsx('rounded-3xl', cls ?? 'w-[550px]')}
	onclose={onClose}
	onclick={() => dialog.close()}
	onkeydown={(e) => {
		if (e.key === 'Escape') dialog.close();
	}}
>
	<div
		role="dialog"
		onclick={(e) => e.stopPropagation()}
		onkeydown={(e) => {
			if (e.key === 'Escape') e.stopPropagation();
		}}
		class="flex mx-auto bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-1 h-full rounded-3xl"
	>
		<div class="flex-1 bg-white rounded-3xl flex flex-col justify-center items-center py-14 px-24">
			{@render header()}
			<hr />
			{@render children()}
		</div>
	</div>
</dialog>

<style>
	dialog::backdrop {
		background: rgba(0, 0, 0, 0.6);
	}

	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	@keyframes zoom {
		from {
			transform: scale(0.95);
		}
		to {
			transform: scale(1);
		}
	}

	dialog[open]::backdrop {
		animation: fade 0.2s ease-out;
	}

	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
</style>

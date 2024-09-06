<script lang="ts">
	interface IProps {
		showModal: boolean;
		onClose: () => void;
	}

	let { showModal = $bindable(), onClose }: IProps = $props();

	let dialog: HTMLDialogElement;

	$effect(() => {
		if (dialog && showModal) dialog.showModal();
		if (dialog && !showModal) dialog.close();
	});
</script>

<dialog
	bind:this={dialog}
	class="rounded-3xl w-[500px]"
	on:close={onClose}
	on:click|self={() => dialog.close()}
>
	<div
		on:click|stopPropagation
		class="flex mx-auto bg-gradient-to-tr from-blue-500 via-orange-400 to-rose-400 p-1 h-full rounded-3xl"
	>
		<div class="flex-1 bg-white rounded-3xl flex justify-center items-center py-10 px-16">
			<slot name="header" />
			<hr />
			<slot />
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

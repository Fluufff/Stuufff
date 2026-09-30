<script lang="ts">
	import { teleport } from '$lib/actions/teleport';
	import Icon from './Icon.svelte';
	import Button from './Button.svelte';

	let {
		children,
		title,
		type = 'info',
		width = '400px',
		open = $bindable(),
		footer
	}: {
		children: any;
		title?: string;
		type?: 'info' | 'success' | 'error' | 'warning';
		width?: string;
		open?: boolean;
		footer?: any;
	} = $props();
	let dialog = $state<HTMLDialogElement>();

	let typeClass = $derived.by(() => {
		switch (type) {
			case 'success':
				return 'bg-green-300 dark:bg-green-700';
			case 'error':
				return 'bg-red-300 dark:bg-red-700';
			case 'warning':
				return 'bg-yellow-300 dark:bg-yellow-700';
			case 'info':
				return 'bg-sky-200 dark:bg-blue-700';
			default:
				return '';
		}
	});

	$effect(() => {
		if (open) dialog?.showModal();
		else dialog?.close();
	});
</script>

{#if open}
	<dialog
		bind:this={dialog}
		class="modal max-h-80vh fixed inset-0 m-auto rounded-lg"
		style="width: {width};"
		onclose={() => (open = false)}
	>
		<div class="header flex items-center justify-between px-8 py-4 {typeClass}">
			{#if title}
				<h2 class="text-lg font-bold">{title}</h2>
			{/if}
			<Button icon="close" label="Close" iconOnly circle onClick={() => (open = false)} />
		</div>
		<div class="body px-8 py-4">
			{@render children()}
		</div>
		{#if footer}
			<div class="footer">
				{@render footer()}
			</div>
		{/if}
	</dialog>
{/if}

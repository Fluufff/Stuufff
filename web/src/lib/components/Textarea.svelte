<script lang="ts">
	import { isBlank } from '$lib/validation';

	let {
		value = $bindable(),
		placeholder,
		label,
		required,
		validate,
		invalidMessage = 'Invalid input',
		className,
		name,
		rows = 7,
		valid = $bindable<boolean>(true)
	}: {
		value?: string;
		placeholder?: string;
		label?: string;
		required?: boolean;
		validate?: (value: string | undefined) => boolean;
		invalidMessage?: string;
		className?: string;
		name?: string;
		rows?: number;
		valid?: boolean;
	} = $props();

	let textareaId = `textarea-${Math.random().toString(36).slice(2, 11)}`;
	let touched = $state(false);

	let isValid = $derived(validate ? validate(value) : !(required && isBlank(value)));
	let showError = $derived(touched && !isValid);

	$effect(() => {
		valid = isValid;
	});
</script>

<div class="my-2">
	{#if label}
		<label class="mb-1 block" for={textareaId}
			>{label}
			{#if required}<span class="text-red-500">*</span>{/if}</label
		>
	{/if}
	<textarea
		bind:value
		{placeholder}
		{name}
		{rows}
		class="input min-h-9 dark:bg-gray-800 dark:text-white! {className} {showError
			? 'border-red-500'
			: ''}"
		id={textareaId}
		{required}
		aria-invalid={showError}
		oninput={() => (touched = true)}></textarea>
	{#if invalidMessage && showError}
		<p class="mt-1 text-sm text-red-500">{invalidMessage}</p>
	{/if}
</div>

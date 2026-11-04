<script lang="ts">
	let {
		value = $bindable(),
		placeholder,
		label,
		required,
		validate,
		invalidMessage = 'Invalid input',
		className,
		name,
		rows = 7
	}: {
		value: string;
		placeholder?: string;
		label?: string;
		required?: boolean;
		validate?: (value: string) => boolean;
		invalidMessage?: string;
		className?: string;
		name?: string;
		rows?: number;
	} = $props();

	let textareaId = `textarea-${Math.random().toString(36).substr(2, 9)}`;
	let hasChanged = false;
	let isValid = $derived.by(() => {
		if (!hasChanged) return true;
		if (validate) return validate(value);
		if (required && !value) return false;

		return true;
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
		class="input min-h-9 dark:bg-gray-800 dark:text-white! {className}"
		id={textareaId}
		{required}
		aria-invalid={!isValid}
		oninput={() => (hasChanged = true)}></textarea>
	{#if invalidMessage && !isValid}
		<p class="mt-1 text-sm text-red-500">{invalidMessage}</p>
	{/if}
</div>

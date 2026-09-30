<script lang="ts">
	let {
		value = $bindable(),
		placeholder,
		type = 'text',
		label,
		required,
		validate,
		invalidMessage = 'Invalid input'
	}: {
		value: string | number;
		placeholder?: string;
		type?: string;
		label?: string;
		required?: boolean;
		validate?: (value: string | number) => boolean;
		invalidMessage?: string;
	} = $props();

	let inputId = `input-${Math.random().toString(36).substr(2, 9)}`;
	let isValid = $derived.by(() => {
		if (validate) return validate(value);
		if (required && !value) return false;

		if (type === 'text' && value && typeof value !== 'string') return false;
		if (type === 'email' && value && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(String(value)))
			return false;
		if (type === 'number' && value && isNaN(Number(value))) return false;
		if (type === 'url' && value && !/^(https?|ftp):\/\/[^\s/$.?#].[^\s]*$/.test(String(value)))
			return false;

		return true;
	});
</script>

<div class="my-2">
	{#if label}
		<label class="mb-1 block" for={inputId}
			>{label}
			{#if required}<span class="text-red-500">*</span>{/if}</label
		>
	{/if}
	<input
		bind:value
		{placeholder}
		{type}
		class="input"
		id={inputId}
		{required}
		aria-invalid={!isValid}
	/>
	{#if invalidMessage && !isValid}
		<p class="mt-1 text-sm text-red-500">{invalidMessage}</p>
	{/if}
</div>

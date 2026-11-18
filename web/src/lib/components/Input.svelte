<script lang="ts">
	import Input from './Input.svelte';
	import { isBlank, isValidEmail, isValidHexColor, isValidUrl } from '$lib/validation';

	let {
		value = $bindable(),
		placeholder,
		type = 'text',
		label,
		required,
		validate,
		invalidMessage = 'Invalid input',
		className,
		min,
		max,
		step = 1,
		plain,
		minlength,
		maxlength,
		valid = $bindable<boolean>(true)
	}: {
		value?: string | number;
		placeholder?: string;
		type?: string;
		label?: string;
		required?: boolean;
		validate?: (value: string | number | undefined) => boolean;
		invalidMessage?: string;
		className?: string;
		min?: number;
		max?: number;
		step?: number;
		plain?: boolean;
		minlength?: number;
		maxlength?: number;
		valid?: boolean;
	} = $props();

	let inputId = `input-${Math.random().toString(36).slice(2, 11)}`;
	let touched = $state(false);

	let isValid = $derived(isValidInput());
	let showError = $derived(touched && !isValid);

	$effect(() => {
		valid = isValid;
	});

	function isValidInput() {
		if (validate) return validate(value);
		if (required && isBlank(value)) return false;
		if (isBlank(value)) return true;

		if (typeof value === 'string') {
			if (minlength && value.length < minlength) return false;
			if (maxlength && value.length > maxlength) return false;
			if (type === 'email' && !isValidEmail(value)) return false;
			if (type === 'url' && !isValidUrl(value)) return false;
			if (type === 'color' && !isValidHexColor(value)) return false;
		}

		if (type === 'number') {
			const number = Number(value);
			if (Number.isNaN(number)) return false;
			if (min !== undefined && number < min) return false;
			if (max !== undefined && number > max) return false;
		}

		return true;
	}
</script>

<div class="my-2">
	{#if label}
		<label class="mb-1 block" for={inputId}
			>{label}
			{#if required}<span class="text-red-500">*</span>{/if}</label
		>
	{/if}
	<div class="flex items-center space-x-2">
		<input
			bind:value
			{placeholder}
			{type}
			{min}
			{max}
			{step}
			{minlength}
			{maxlength}
			class="input min-h-9 dark:bg-gray-800 dark:text-white! {className} {plain
				? 'plain'
				: ''} {showError ? 'border-red-500' : ''}"
			id={inputId}
			{required}
			aria-invalid={showError}
			oninput={() => (touched = true)}
		/>
		{#if type === 'color'}
			<Input type="text" bind:value plain maxlength={7} required />
		{/if}
	</div>
	{#if invalidMessage && showError}
		<p class="mt-1 text-sm text-red-500">{invalidMessage}</p>
	{/if}
</div>

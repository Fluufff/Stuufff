<script lang="ts" generics="T extends string">
	let {
		value = $bindable<T>(),
		options,
		label,
		className,
		optionType = 'default'
	}: {
		value: T;
		options: readonly T[] | readonly { value: T; label: string }[];
		label?: string;
		className?: string;
		optionType?: 'default' | 'object';
	} = $props();

	let inputId = `input-${Math.random().toString(36).substr(2, 9)}`;
</script>

<div class="my-2">
	{#if label}
		<label class="mb-1 block" for={inputId}>{label}</label>
	{/if}
	<select
		bind:value
		id={inputId}
		class="input min-h-9 cursor-pointer px-3 py-2 dark:bg-gray-800 dark:text-white! {className}"
	>
		{#each options as option (typeof option === 'object' ? option?.value : option)}
			<option value={typeof option === 'object' ? option?.value : option}
				>{typeof option === 'object' ? option?.label : option}</option
			>
		{/each}
	</select>
</div>

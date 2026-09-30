<script lang="ts">
	let {
		checked = $bindable<boolean>(),
		disabled = false,
		activeLabel,
		inactiveLabel,
		onChange
	}: {
		checked: boolean;
		disabled?: boolean;
		activeLabel?: string;
		inactiveLabel?: string;
		onChange?: () => void;
	} = $props();

	const id = Math.random().toString(36).substr(2, 9);
</script>

<div class="my-2 flex items-center">
	<label for={id} class="relative inline-block h-6 w-11 cursor-pointer">
		<input
			type="checkbox"
			{id}
			class="peer sr-only"
			bind:checked
			{disabled}
			onchange={() => onChange?.()}
		/>
		<span
			class="absolute inset-0 rounded-full bg-gray-200 transition-colors duration-200 ease-in-out peer-checked:bg-blue-600 peer-disabled:pointer-events-none peer-disabled:opacity-50 dark:bg-neutral-600 dark:peer-checked:bg-blue-500"
		></span>
		<span
			class="absolute inset-s-0.5 top-1/2 size-5 -translate-y-1/2 rounded-full bg-white shadow-sm transition-transform duration-200 ease-in-out peer-checked:translate-x-full"
		></span>
	</label>
	{#if activeLabel && inactiveLabel}
		<label class="ml-2 text-sm" for={id}>
			{#if checked}
				{activeLabel}
			{:else}
				{inactiveLabel}
			{/if}
		</label>
	{:else if activeLabel || inactiveLabel}
		<label class="ml-2 text-sm" for={id}>
			{#if activeLabel}
				{activeLabel}
			{:else}
				{inactiveLabel}
			{/if}
		</label>
	{/if}
</div>

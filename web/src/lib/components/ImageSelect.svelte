<script lang="ts">
	import Icon from './Icon.svelte';

	let {
		images,
		groupName,
		group = $bindable(),
		type = 'radio' as 'radio' | 'checkbox',
		selectedLabel = 'Selected',
		unselectedLabel = 'Select'
	}: {
		images: { id: string; url: string | undefined }[];
		groupName: string;
		group: string | string[] | null;
		type?: 'radio' | 'checkbox';
		selectedLabel?: string;
		unselectedLabel?: string;
	} = $props();
</script>

<div class="flex flex-wrap gap-3">
	{#each images as { id, url } (id)}
		{@const isSelected =
			type === 'radio' ? group === id : Array.isArray(group) && group.includes(id)}
		<label class="relative flex cursor-pointer flex-col items-center gap-1 text-sm">
			{#if type === 'radio'}
				<input
					type="radio"
					name={groupName}
					value={id}
					bind:group
					aria-label="Select image"
					class="hidden"
				/>
			{:else if type === 'checkbox'}
				<input
					type="checkbox"
					name={groupName}
					value={id}
					bind:group
					aria-label="Select image"
					class="hidden"
				/>
			{/if}
			{#if url}
				<img
					src={url}
					alt="Image option {id}"
					class="transition-filter h-16 w-16 rounded border border-gray-300 object-cover duration-300 hover:brightness-75"
				/>
			{:else}
				<span class="flex h-16 w-16 items-center justify-center border border-gray-300"
					>Unavailable</span
				>
			{/if}

			{#if isSelected}
				<div
					class="absolute top-0 left-0 flex h-16 w-16 items-center justify-center rounded border-4 border-slate-500 bg-slate-500/20"
				>
					<Icon name="check" size={32} class="text-white text-shadow-lg"></Icon>
				</div>
			{/if}

			<span>{isSelected ? selectedLabel : unselectedLabel}</span>
		</label>
	{/each}
</div>

<script lang="ts">
	let {
		files = $bindable<FileList | undefined>(),
		label = 'Choose files',
		accept,
		multiple = false,
		required = false,
		disabled = false,
		preview = false,
		onchange
	}: {
		files?: FileList;
		label?: string;
		accept?: string;
		multiple?: boolean;
		required?: boolean;
		disabled?: boolean;
		preview?: boolean;
		onchange?: (event: Event) => void;
	} = $props();

	let selectedFiles = $derived(files ? Array.from(files) : []);
</script>

<div class="my-2 flex flex-wrap items-center gap-2">
	<label
		class="relative inline-flex cursor-pointer items-center gap-2 rounded border-1 border-green-600 px-4 py-2 transition-colors duration-300 ease-in-out hover:bg-green-600 focus:bg-green-600 has-[:disabled]:cursor-not-allowed has-[:disabled]:opacity-60"
	>
		{label}
		<input
			type="file"
			class="absolute inset-0 h-0 w-0 cursor-pointer opacity-0 disabled:cursor-not-allowed"
			bind:files
			{accept}
			{multiple}
			{required}
			{disabled}
			{onchange}
		/>
	</label>
	{#if selectedFiles.length}
		<ul class="min-w-0 text-sm" aria-live="polite">
			{#each selectedFiles as file (file.name + file.lastModified)}
				<li class="max-w-full truncate">{file.name}</li>
			{/each}
		</ul>
	{:else}
		<span class="text-sm">No files selected</span>
	{/if}
	{#if preview && selectedFiles.length}
		<div class="mt-2 flex flex-wrap gap-2">
			{#each selectedFiles as file (file.name + file.lastModified)}
				<div class="h-20 w-20 overflow-hidden rounded border border-gray-300">
					<img src={URL.createObjectURL(file)} alt={file.name} class="h-full w-full object-cover" />
				</div>
			{/each}
		</div>
	{/if}
</div>

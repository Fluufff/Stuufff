<script lang="ts">
	import { processFiles } from '$lib/utils/resizeImage';

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

	let previewUrls = $derived.by(() =>
		preview ? selectedFiles.map((file) => URL.createObjectURL(file)) : []
	);

	$effect(() => {
		const urls = previewUrls;
		return () => urls.forEach((url) => URL.revokeObjectURL(url));
	});

	function removeFile(index: number) {
		if (!files) return;
		const dt = new DataTransfer();
		selectedFiles.forEach((file, i) => {
			if (i !== index) dt.items.add(file);
		});
		files = dt.files;
	}

	async function handleInput(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		files = await processFiles(input.files);
		onchange?.(event);
	}
</script>

<div class="my-2 flex flex-col gap-2">
	<div class="flex flex-wrap items-center gap-2">
		<label
			class="relative inline-flex cursor-pointer items-center gap-2 rounded border-1 border-green-600 px-4 py-2 transition-colors duration-300 ease-in-out hover:bg-green-600 focus:bg-green-600 has-[:disabled]:cursor-not-allowed has-[:disabled]:opacity-60"
		>
			{label}
			<input
				type="file"
				class="absolute inset-0 h-0 w-0 cursor-pointer opacity-0 disabled:cursor-not-allowed"
				{accept}
				{multiple}
				{required}
				{disabled}
				onchange={handleInput}
			/>
		</label>
		<span class="text-sm" aria-live="polite">
			{selectedFiles.length
				? `${selectedFiles.length} ${selectedFiles.length === 1 ? 'file' : 'files'} selected`
				: 'No files selected'}
		</span>
	</div>
	{#if selectedFiles.length}
		{#if preview}
			<ul class="flex flex-wrap gap-2">
				{#each selectedFiles as file, i (file.name + file.lastModified)}
					<li class="flex w-20 flex-col items-center gap-1 text-xs">
						<div class="relative h-20 w-20 overflow-hidden rounded border border-gray-300">
							<img src={previewUrls[i]} alt={file.name} class="h-full w-full object-cover" />
							<button
								type="button"
								aria-label="Remove {file.name}"
								class="absolute top-0 right-0 flex h-5 w-5 cursor-pointer items-center justify-center rounded-bl bg-red-800/80 text-white hover:bg-red-700"
								onclick={() => removeFile(i)}
							>
								×
							</button>
						</div>
						<span class="w-full truncate text-center" title={file.name}>{file.name}</span>
					</li>
				{/each}
			</ul>
		{:else}
			<ul class="flex flex-wrap gap-2" aria-live="polite">
				{#each selectedFiles as file, i (file.name + file.lastModified)}
					<li
						class="flex max-w-full items-center gap-1 rounded border border-gray-300 bg-gray-100 px-2 py-0.5 text-sm"
					>
						<span class="max-w-40 truncate" title={file.name}>{file.name}</span>
						<button
							type="button"
							aria-label="Remove {file.name}"
							class="flex cursor-pointer items-center justify-center text-red-700 hover:text-red-500"
							onclick={() => removeFile(i)}
						>
							×
						</button>
					</li>
				{/each}
			</ul>
		{/if}
	{/if}
</div>

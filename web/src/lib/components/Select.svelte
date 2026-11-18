<script lang="ts">
	import { isBlank } from '$lib/validation';
	import {
		Combobox,
		Portal,
		useListCollection,
		type ComboboxRootProps
	} from '@skeletonlabs/skeleton-svelte';

	let {
		value = $bindable<string | string[]>(),
		options,
		label,
		className,
		required,
		multiple = false,
		invalidMessage = 'Invalid input',
		valid = $bindable<boolean>(true),
		noPills
	}: {
		value: string | string[];
		options: readonly string[] | readonly { value: string; label: string }[];
		label?: string;
		className?: string;
		required?: boolean;
		multiple?: boolean;
		invalidMessage?: string;
		valid?: boolean;
		noPills?: boolean;
	} = $props();

	type LabeledOption = { value: string; label: string; color?: string };

	function isLabeledOption(option: string | LabeledOption): option is LabeledOption {
		return typeof option === 'object' && option !== null && 'value' in option && 'label' in option;
	}

	let normalizedOptions = $derived(
		options.map((option) =>
			isLabeledOption(option) ? option : { value: option, label: String(option), color: undefined }
		)
	);

	let search = $state('');

	let filteredOptions = $derived.by(() => {
		const needle = search.trim().toLowerCase();
		if (!needle) return normalizedOptions;
		return normalizedOptions.filter((option) => option.label.toLowerCase().includes(needle));
	});

	let collection = $derived(
		useListCollection({
			items: filteredOptions,
			itemToString: (item) => item.label,
			itemToValue: (item) => String(item.value)
		})
	);

	let selectedValues = $derived.by(() => {
		if (Array.isArray(value)) return value.map(String);
		return value ? [String(value)] : [];
	});

	let labelByValue = $derived(
		new Map(normalizedOptions.map((option) => [option.value, option.label]))
	);

	let colorByValue = $derived(
		new Map(normalizedOptions.map((option) => [option.value, option.color]))
	);

	let inputId = `input-${Math.random().toString(36).slice(2, 11)}`;
	let touched = $state(false);
	let inputValue = $state('');

	let isValid = $derived(!(required && isBlank(value)));
	let showError = $derived(touched && !isValid);

	$effect(() => {
		if (valid !== isValid) valid = isValid;
	});

	$effect(() => {
		if (multiple) return;
		const selected = selectedValues[0];
		inputValue = selected ? (labelByValue.get(selected) ?? '') : '';
	});

	const handleValueChange: NonNullable<ComboboxRootProps['onValueChange']> = (details) => {
		touched = true;
		if (multiple) {
			value = details.value;
		} else {
			value = details.value[0] ?? '';
		}
	};

	const handleInputValueChange: NonNullable<ComboboxRootProps['onInputValueChange']> = (
		details
	) => {
		inputValue = details.inputValue;
		if (details.reason === 'input-change') {
			search = details.inputValue;
		}
	};

	const handleOpenChange: NonNullable<ComboboxRootProps['onOpenChange']> = (details) => {
		if (details.open) search = '';
	};

	function removeSelectedValue(removedValue: string) {
		if (!multiple) return;
		const current = Array.isArray(value) ? value : value ? [value] : [];
		value = current.filter((item) => item !== removedValue);
	}
</script>

<div>
	{#if label}
		<label class="mb-1 block" for={inputId}
			>{label}
			{#if required}<span class="text-red-500">*</span>{/if}</label
		>
	{/if}
	<Combobox
		class={className ?? ''}
		placeholder="Search..."
		{collection}
		value={selectedValues}
		{multiple}
		{inputValue}
		openOnClick
		ids={{ input: inputId }}
		{required}
		invalid={showError}
		onValueChange={handleValueChange}
		onInputValueChange={handleInputValueChange}
		onOpenChange={handleOpenChange}
	>
		<Combobox.Control
			class="input flex! min-h-9 cursor-pointer flex-wrap items-center gap-1 px-3 py-2 dark:bg-gray-800 {showError
				? 'ring-red-500!'
				: ''}"
		>
			<Combobox.Input class="input input-ghost min-w-0 flex-1 ring-0! dark:text-white!" />
			<Combobox.Trigger class="shrink-0 cursor-pointer px-1" />
		</Combobox.Control>
		<Portal>
			<Combobox.Positioner class="z-50">
				<Combobox.Content
					class="max-h-64 overflow-y-auto rounded border border-gray-300 bg-white p-1 shadow-lg dark:border-gray-600 dark:bg-gray-800"
				>
					{#each filteredOptions as item (item.value)}
						<Combobox.Item
							{item}
							class="flex cursor-pointer items-center gap-2 rounded-sm px-2 py-1.5 text-sm has-[>*[data-state='checked']]:bg-lime-300 has-[>*[data-state='checked']]:text-black data-highlighted:bg-gray-200 has-[>*[data-state='checked']]:dark:bg-lime-900 has-[>*[data-state='checked']]:dark:text-white dark:data-highlighted:bg-gray-600"
						>
							<Combobox.ItemText class="min-w-0 flex-1 truncate">{item.label}</Combobox.ItemText>
							<Combobox.ItemIndicator class="shrink-0" />
						</Combobox.Item>
					{/each}
					{#if filteredOptions.length === 0}
						<li class="px-2 py-1.5 text-sm text-gray-500">No matches found</li>
					{/if}
				</Combobox.Content>
			</Combobox.Positioner>
		</Portal>
	</Combobox>
	{#if !noPills && multiple && selectedValues.length}
		<div class="mt-3 flex space-x-3">
			{#if multiple}
				{#each selectedValues as selectedValue (selectedValue)}
					<span
						class="flex items-center gap-3 rounded-sm px-3 py-1.5 {'bg-' +
							(`[${colorByValue.get(selectedValue)}]` || 'gray-200')} {'dark:bg-' +
							(`[${colorByValue.get(selectedValue)}]` || 'gray-600')}"
						style="background-color: {colorByValue.get(selectedValue) || 'var(--bg-gray-200)'};"
					>
						{labelByValue.get(selectedValue) ?? selectedValue}
						<button
							type="button"
							class="cursor-pointer leading-none"
							aria-label="Remove {labelByValue.get(selectedValue) ?? selectedValue}"
							onmousedown={(event) => event.preventDefault()}
							onclick={(event) => {
								event.stopPropagation();
								removeSelectedValue(selectedValue);
							}}
						>
							×
						</button>
					</span>
				{/each}
			{/if}
		</div>
	{/if}
	{#if invalidMessage && showError}
		<p class="mt-1 text-sm text-red-500">{invalidMessage}</p>
	{/if}
</div>

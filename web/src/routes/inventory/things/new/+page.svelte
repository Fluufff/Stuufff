<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { places } from '$lib/data/places.svelte';
	import Textarea from '$lib/components/Textarea.svelte';
	import { labels } from '$lib/data/label.svelte';
	import { departments } from '$lib/data/department.svelte';
	import Input from '$lib/components/Input.svelte';
	import FileInput from '$lib/components/FileInput.svelte';
	import Select from '$lib/components/Select.svelte';
	import Button from '$lib/components/Button.svelte';
	import { createThing, fetchThingExpanded, uploadImage } from '$lib/data/things.svelte';
	import NewLabel from './newLabel.svelte';

	let name = $state('');
	let description = $state('');
	let count = $state(1);
	let images = $state(new DataTransfer().files);
	let place = $state('');
	let label = $state<string | string[]>('');
	let department = $state('');

	let createLabel = $state(false);

	let saving = $state(false);

	let nameValid = $state(true);
	let descriptionValid = $state(true);
	let countValid = $state(true);
	let placeValid = $state(true);
	let departmentValid = $state(true);
	let labelValid = $state(true);

	let labelOptions = $derived(
		[
			...labels
				.entries()
				.map(([id, label]) => ({ value: String(id), label: label.name, color: label.color }))
		].sort((a, b) => a.label.localeCompare(b.label))
	);

	let canSave = $derived(
		nameValid && descriptionValid && countValid && placeValid && departmentValid && labelValid
	);

	async function save() {
		if (!canSave) return;
		saving = true;
		try {
			const { id } = await createThing({
				name,
				description,
				count,
				in_place: place ? Number(place) : undefined,
				department: department ? Number(department) : null,
				label_ids: (Array.isArray(label) ? label : [label]).map((lab) => Number(lab))
			});

			await Promise.all(Array.from(images).map((file) => uploadImage(id, file)));
			await fetchThingExpanded(id);
			goto(resolve(`/inventory`));
		} catch (error) {
			console.error('Failed to save thing:', error);
		} finally {
			saving = false;
		}
	}
</script>

<section class="container mx-auto max-w-2xl px-5">
	{#await Promise.all([places, labels, departments])}
		<p>loading things...</p>
	{:then [places, labels, departments]}
		<fieldset class="flex flex-col space-y-4 rounded border p-3">
			<legend class="px-3 text-2xl font-bold">New Thing</legend>
			<Input label="Name" bind:value={name} bind:valid={nameValid} required />
			<Textarea
				label="Description"
				rows={7}
				bind:value={description}
				bind:valid={descriptionValid}
				required
			/>
			<Input
				label="Count"
				type="number"
				bind:value={count}
				bind:valid={countValid}
				min={1}
				required
			/>
			<FileInput label="Images" bind:files={images} accept="image/*" multiple preview />
			<Select
				label="Place"
				bind:value={place}
				bind:valid={placeValid}
				required
				options={Object.entries(places).map(([id, place]) => ({ value: id, label: place.name }))}
			/>
			<Select
				label="Department"
				bind:value={department}
				bind:valid={departmentValid}
				required
				options={Object.entries(departments).map(([id, department]) => ({
					value: id,
					label: department.name
				}))}
			/>
			<Select label="Label" bind:value={label} multiple options={labelOptions} />
			<Button label="New Label" onClick={() => (createLabel = true)} fullWidth />
			<div class="flex space-x-3">
				<Button label="Cancel" onClick={() => goto(resolve(`/inventory`))} fullWidth />
				<Button
					fullWidth
					onClick={save}
					disabled={!canSave || saving}
					label={saving ? 'Saving...' : 'Save'}
					status="success"
				/>
			</div>
		</fieldset>
		<NewLabel open={createLabel} />
	{:catch err}
		<p>loading failed: {err}</p>
	{/await}
</section>

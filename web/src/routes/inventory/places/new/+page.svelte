<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import Button from '$lib/components/Button.svelte';
	import FileInput from '$lib/components/FileInput.svelte';
	import ImageSelect from '$lib/components/ImageSelect.svelte';
	import Input from '$lib/components/Input.svelte';
	import Select from '$lib/components/Select.svelte';
	import Textarea from '$lib/components/Textarea.svelte';
	import { departments } from '$lib/data/department.svelte';
	import {
		places,
		createPlace,
		updatePlace,
		uploadPlaceImage,
		fetchPlace,
		type Place,
		deletePlaceImage
	} from '$lib/data/places.svelte';
	import Switch from '$lib/components/Switch.svelte';
	import { canEdit } from '$lib/auth.svelte';

	let name = $state('');
	let images = $state(new DataTransfer().files);
	let place = $state('');
	let department = $state('');
	let description = $state('');
	let mainImg = $state<string | null>(null);

	const editId = $derived.by(() => {
		if (!canEdit()) return undefined;
		const raw = page.url.searchParams.get('id');
		if (raw === null) return undefined;
		const id = Number(raw);
		return Number.isInteger(id) && id > 0 ? id : undefined;
	});
	const isEditing = $derived(editId !== undefined);

	let initialPlace = $state<Place | undefined>(undefined);
	let initialError = $state('');
	let loadedId = $state<number | undefined>(undefined);

	let wantToDelete = $state<boolean>(false);
	let imagesToDelete = $state<string[]>([]);
	let wantToUpload = $state<boolean>(false);

	$effect(() => {
		if (editId === loadedId) return;
		loadedId = editId;
		name = '';
		images = new DataTransfer().files;
		place = '';
		department = '';
		description = '';
		mainImg = null;
		initialPlace = undefined;
		initialError = '';
		if (editId === undefined) return;
		const found = places[editId];
		if (!found) {
			initialError = `Place ${editId} not found`;
			return;
		}
		initialPlace = found;
		name = found.name;
		description = found.description ?? '';
		place = found.in_place != undefined ? String(found.in_place) : '';
		department = found.in_department != undefined ? String(found.in_department) : '';
		mainImg = found.main_img ?? null;
	});

	let cansave = $derived(name != '' && description != '');
	let saving = $state(false);

	const save = async () => {
		if (!cansave) return;
		saving = true;
		try {
			if (isEditing && editId !== undefined) {
				await updatePlace({
					id: editId,
					name,
					description,
					in_place: place ? Number(place) : undefined,
					in_department: department ? Number(department) : undefined,
					main_img: mainImg,
					image_ids: initialPlace?.image_ids ?? [],
					reservations: initialPlace?.reservations ?? []
				});
				await Promise.all(Array.from(images).map((file) => uploadPlaceImage(editId, file)));
				if (wantToDelete && imagesToDelete.length > 0) {
					await Promise.all(imagesToDelete.map((id) => deletePlaceImage(editId, id)));
				}
				await fetchPlace(editId);
			} else {
				const { id } = await createPlace({
					name,
					description,
					in_place: place ? Number(place) : undefined,
					in_department: department ? Number(department) : undefined,
					image_ids: [],
					reservations: []
				});
				await Promise.all(Array.from(images).map((file) => uploadPlaceImage(id, file)));
				await fetchPlace(id);
			}
			saving = false;
			goto(resolve(`/inventory`));
		} catch (err) {
			console.error('cannot save', err);
			saving = false;
		}
	};
</script>

<section class="container mx-auto max-w-2xl px-5">
	{#await Promise.all([places, departments])}
		<p>loading things...</p>
	{:then [places, departments]}
		{#if isEditing && !initialPlace && !initialError}
			<p>loading place...</p>
		{:else if initialError}
			<p>loading failed: {initialError}</p>
		{:else}
			<fieldset class="flex flex-col space-y-4 rounded border p-3">
				<legend class="px-3 text-2xl font-bold">{isEditing ? 'Edit Place' : 'New Place'}</legend>
				<Input label="Name" bind:value={name} required placeholder="Enter the name of the place" />
				<Textarea
					rows={7}
					name="description"
					bind:value={description}
					required
					label="Description"
					placeholder="Enter the description of the place"
				/>
				<Select
					label="Location"
					bind:value={place}
					options={[
						{ value: '', label: '-' },
						...Object.entries(places)
							.filter(([id, place]) => (editId ? place.id !== Number(editId) : true))
							.map(([id, place]) => ({ value: id, label: place.name }))
					]}
				/>
				<Select
					label="Department"
					bind:value={department}
					options={[
						{ value: '', label: '-' },
						...Object.entries(departments).map(([id, department]) => ({
							value: id,
							label: department.name
						}))
					]}
				/>
				{#if isEditing && initialPlace && initialPlace.image_ids.length > 0 && Object.keys(initialPlace.image_urls ?? {}).length > 0}
					<fieldset class="my-3">
						<legend class="mb-2 font-medium">Select main image</legend>
						<ImageSelect
							images={initialPlace.image_ids.map((id) => ({
								id,
								url: initialPlace?.image_urls?.[id]
							}))}
							groupName="main-image"
							bind:group={mainImg}
							selectedLabel="Main"
							unselectedLabel="Select"
						/>
					</fieldset>
					<Switch bind:checked={wantToDelete} activeLabel="Delete image(s)" />
					{#if wantToDelete}
						<fieldset class="my-3">
							<legend>Select images to delete</legend>
							<ImageSelect
								images={initialPlace.image_ids.map((id) => ({
									id,
									url: initialPlace?.image_urls?.[id]
								}))}
								groupName="delete-images"
								bind:group={imagesToDelete}
								selectedLabel="Delete"
								unselectedLabel="Keep"
								type="checkbox"
							/>
						</fieldset>
					{/if}
					<Switch bind:checked={wantToUpload} activeLabel="Upload new image(s)" />
					{#if wantToUpload}
						<FileInput bind:files={images} accept="image/*" multiple preview />
					{/if}
				{:else}
					<FileInput bind:files={images} accept="image/*" multiple preview />
				{/if}
				<div class="flex space-x-3">
					<Button label="Cancel" onClick={() => goto(resolve(`/inventory`))} fullWidth />
					<Button
						fullWidth
						onClick={save}
						disabled={!cansave || saving}
						label={saving ? 'Saving...' : 'Save'}
						status="success"
					/>
				</div>
			</fieldset>
		{/if}
	{:catch err}
		<p>loading failed: {err}</p>
	{/await}
</section>

<script lang="ts">
	import { onMount } from 'svelte';

	import { updateDepartment, deleteDepartment, type Department } from '$lib/data/department.svelte';
	import Button from '$lib/components/Button.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import Input from '$lib/components/Input.svelte';
	import FileInput from '$lib/components/FileInput.svelte';
	import Switch from '$lib/components/Switch.svelte';
	import ImageSelect from '$lib/components/ImageSelect.svelte';
	import { canEdit } from '$lib/auth.svelte';

	let { departments }: { departments: Record<number, Department> } = $props();

	onMount(() => {
		console.log('Department tab mounted', departments);
	});

	let showEditModal = $state(false);
	let wantNewImage = $state(false);
	let wantDeleteImage = $state(false);
	let imagesToDelete = $state<string[]>([]);
	let saveError = $state('');
	let editDepartment = $state<
		(Department & { newImage?: FileList; imagesToDelete?: string[] }) | undefined
	>(undefined);

	function onShowEditModal(departmentID: number) {
		console.log('Edit modal shown', departmentID);
		const foundDepartment = Object.values(departments).find(
			(department) => department.id === departmentID
		);
		editDepartment = foundDepartment
			? { ...foundDepartment, newImage: undefined, imagesToDelete: [] }
			: undefined;
		showEditModal = true;
		wantNewImage = false;
		wantDeleteImage = false;
		imagesToDelete = [];
		saveError = '';
	}

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		if (!editDepartment) return;
		saveError = '';
		editDepartment.imagesToDelete = imagesToDelete;
		try {
			await updateDepartment(editDepartment);
			showEditModal = false;
			editDepartment = undefined;
			wantNewImage = false;
			wantDeleteImage = false;
			imagesToDelete = [];
		} catch (error) {
			saveError = error instanceof Error ? error.message : 'Failed to update department';
		}
	}

	$effect(() => {
		if (!showEditModal) return;
		if (!editDepartment) return;
		if (wantNewImage) return;
		else editDepartment.newImage = undefined;
	});
</script>

<table class="min-w-full table-auto border-collapse">
	<thead>
		<tr class="border-b border-gray-300">
			<th class="w-fit p-2 text-left">Image</th>
			<th class=" p-2 text-left">Name</th>
			<th></th>
		</tr>
	</thead>
	<tbody>
		{#each Object.entries(departments) as [id, department] (id)}
			<tr
				class="cursor-pointer border-gray-200 transition-colors duration-300 ease-in-out not-last:border-b hover:bg-gray-100"
			>
				<td class="h-20 max-h-20 w-20 max-w-20 p-2">
					{#if department.main_img}
						<img class="max-h-16 max-w-16" src={department.main_url} alt="" />
					{:else}
						<div class="flex items-center justify-center">
							<span class="icon-[material-symbols--no-photography-outline] bg-gray-200 text-[20px]"
							></span>
						</div>
					{/if}
				</td>
				<td class="p-2">{department.name}</td>
				{#if canEdit()}
					<td class="max-w-20 p-2">
						<Button
							icon="edit"
							label="Edit"
							iconOnly
							circle
							onClick={() => {
								onShowEditModal(department.id);
							}}
						/>
					</td>
				{/if}
			</tr>
		{/each}
	</tbody>
</table>

<Modal bind:open={showEditModal} title="Editing department: {editDepartment?.name}" width="500px">
	{#if editDepartment}
		<form onsubmit={submit}>
			<Input bind:value={editDepartment.name} placeholder="Department Name" label="Name" required />
			{#if editDepartment.image_ids.length > 1}
				<fieldset class="my-3">
					<legend class="mb-2 font-medium">Select main image</legend>
					<ImageSelect
						images={editDepartment.image_ids.map((id) => ({
							id,
							url: editDepartment?.image_urls[id]
						}))}
						groupName="main-image"
						bind:group={editDepartment.main_img as string | null}
						selectedLabel="Main"
						unselectedLabel="Select"
					/>
				</fieldset>
			{/if}

			<Switch bind:checked={wantNewImage} activeLabel="Add new image" />
			{#if wantNewImage}
				<FileInput bind:files={editDepartment.newImage} label="Image" accept="image/*" preview />
			{/if}

			<Switch
				bind:checked={wantDeleteImage}
				activeLabel="Delete image(s)"
				onChange={() => (imagesToDelete = [])}
			/>
			{#if wantDeleteImage && editDepartment.image_ids.length > 0}
				<fieldset class="my-3">
					<legend class="mb-2 font-medium">Select images to delete</legend>
					<ImageSelect
						images={editDepartment.image_ids.map((id) => ({
							id,
							url: editDepartment?.image_urls[id]
						}))}
						groupName="delete-images"
						bind:group={imagesToDelete}
						type="checkbox"
						selectedLabel="Delete"
					/>
				</fieldset>
			{/if}

			{#if saveError}
				<p class="my-2 text-sm text-red-700" role="alert">{saveError}</p>
			{/if}
			<div class="flex gap-3">
				<Button
					label="Delete"
					type="button"
					fullWidth
					status="danger"
					onClick={async () => {
						await deleteDepartment(editDepartment!.id);
						showEditModal = false;
					}}
				/>
				<Button label="Save" type="submit" fullWidth />
			</div>
		</form>
	{/if}
</Modal>

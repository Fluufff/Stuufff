<script lang="ts">
	import { goto } from '$app/navigation';
	import { newPlace } from '$lib/data';
	import { resolve } from '$app/paths';
	import { places, departments } from '$lib/data';

	let name = $state('');
	let images = $state(new DataTransfer().files);
	let place = $state('');
	let department = $state('');
	let description = $state('');

	let cansave = $derived(name != '' && description != '');
	let saving = $state(false);
</script>

<section class="max-w-6xl grid grid-cols-[auto_1fr] gap-2">
	{#await Promise.all([places, departments])}
		<p>loading things...</p>
	{:then [places, departments]}
		<p>images</p>
		<div>
			<label class="btn bg-green-800 hover:bg-green-700">
				Select images
				<input type="file" accept="image/*" hidden multiple bind:files={images} />
			</label>
			<span>{images.length ? images.length : 'no'} images selected</span>
		</div>

		<p>name</p>
		<input type="text" name="name" bind:value={name} required />

		<p>description</p>
		<textarea rows="7" name="description" bind:value={description} required></textarea>

		<p>location</p>
		<select bind:value={place}>
			<option value="">-</option>
			{#each Object.entries(places) as [id, place] (id)}
				<option value={id}>{place.name}</option>
			{/each}
		</select>

		<p>department</p>
		<select bind:value={department}>
			<option value="">-</option>
			{#each Object.entries(departments) as [id, department] (id)}
				<option value={id}>{department.name}</option>
			{/each}
		</select>

		<p></p>
		<button
			aria-label="save"
			onclick={() => {
				if (cansave) {
					saving = true;
					newPlace({
						name,
						description,
						in_place: place ? Number(place) : undefined,
						in_department: department ? Number(department) : undefined
					})
						.then(({ id }) => {
							return Promise.all(
								Array.from(images).map((file) =>
									fetch(`/api/v1/places/${id}/images`, {
										method: 'POST',
										headers: {
											'Content-Type': file.type
										},
										body: file
									}).catch((err) => console.error('image failed to upload', err))
								)
							).then(() => goto(resolve(`/inventory`)));
						})
						.catch((err) => console.error('cannot save', err));
				}
			}}
			class={[
				cansave && !saving ? 'bg-green-800 hover:bg-green-700' : 'bg-gray-600',
				'rounded-md p-2'
			]}
			disabled={!cansave || saving}>{saving ? 'Saving...' : 'Save'}</button
		>
	{:catch err}
		<p>loading failed: {err}</p>
	{/await}
</section>

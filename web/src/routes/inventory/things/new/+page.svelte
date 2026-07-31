<script lang="ts">
	import { goto } from '$app/navigation';
	import { newThing } from '$lib/data';
	import { resolve } from '$app/paths';
	import { places } from '$lib/data';

	let name = $state('');
	let description = $state('');
	let count = $state(1);
	let images = $state(new DataTransfer().files);
	let place = $state('');

	let cansave = $derived(name != '' && description != '' && count > 0);
	let saving = $state(false);
</script>

<section class="max-w-6xl grid grid-cols-[auto_1fr] gap-2">
	{#await Promise.all([places])}
		<p>loading things...</p>
	{:then [places]}
		<p>images</p>
		<div>
			<label class="btn bg-green-800 hover:bg-green-700">
				Select images
				<input type="file" accept="image/*" hidden multiple bind:files={images} />
			</label>
			<span>{images.length ? images.length : 'no'} images selected</span>
		</div>

		<p>count</p>
		<input type="number" name="count" bind:value={count} min="1" />

		<p>name</p>
		<input type="text" name="name" bind:value={name} required />

		<p>location</p>
		<select bind:value={place}>
			<option value="">-</option>
			{#each Object.entries(places) as [id, place] (id)}
				<option value={id}>{place.name}</option>
			{/each}
		</select>

		<p>description</p>
		<textarea rows="7" name="description" bind:value={description} required></textarea>

		<p></p>
		<button
			aria-label="save"
			onclick={() => {
				if (cansave) {
					saving = true;
					newThing({
						name,
						description,
						count,
						in_place: place ? Number(place) : undefined
					})
						.then(({ id }) => {
							return Promise.all(
								Array.from(images).map((file) =>
									fetch(`/api/v1/things/${id}/images`, {
										method: 'POST',
										headers: {
											'Content-Type': file.type
										},
										body: file
									}).catch((err) => console.error('image failed to upload', err))
								)
							).then(() => goto(resolve(`/inventory/things/${id}`)));
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

<script lang="ts">
	import { page } from '$app/state';
	import {
		fetchThing,
		places,
		labels,
		type Thing,
		updateThing,
		departments,
		newLabel
	} from '$lib/data';

	let editLabels = $state(false);
	let labelSearch = $state('');
	const filteredLabels = $derived.by(() => {
		const s = labelSearch.toLowerCase();

		return labels.then((labels) =>
			Object.entries(labels)
				.filter(([_, label]) => {
					const t = label.name.toLowerCase() + (label.description || '').toLowerCase();
					console.log(t);
					return !s || t.indexOf(s) != -1;
				})
				.map(([id, _]) => Number(id))
		);
	});

	const thing = $state({
		value: null as Thing | null,
		original: null as Thing | null,
		error: null as string | null,
		id: null as number | null
	});

	$effect(() => {
		const id = Number(page.params.id);
		if (isNaN(id)) {
			thing.error = 'not a valid thing id';
			return;
		}
		thing.error = null;

		if (thing.id == id) {
			return;
		}

		thing.id = id;

		fetchThing(id).then((resp) => {
			thing.value = resp;
			thing.original = structuredClone(resp);
		});
	});

	const edited = $derived.by(() => {
		return JSON.stringify(thing.value) != JSON.stringify(thing.original);
	});

	const saving = $state({ thing: false, images: false });

	const saveImages = (event: Event) => {
		const input = event.currentTarget as HTMLInputElement;
		if (!input.files) return;

		saving.images = true;

		for (const file of input.files) {
			fetch(`/api/v1/things/${thing.id}/images`, {
				method: 'POST',
				headers: {
					'Content-Type': file.type
				},
				body: file
			});
		}

		input.value = '';
		saving.images = false;
	};

	const toggleLabel = (thing: Thing, label_id: number) => {
		const i = thing.label_ids.indexOf(label_id);
		console.log(label_id, i);
		if (i == -1) {
			thing.label_ids.push(label_id);
		} else {
			thing.label_ids.splice(i, 1);
		}
	};
</script>

<section class="grid-area max-w-6xl flex flex-col m-4 gap-2">
	{#if thing.error}
		<p>loading failed: {thing.error}</p>
	{:else if thing.value == null || thing.original == null}
		<p>loading thing {page.params.id}</p>
	{:else}
		{#await Promise.all([places, labels, filteredLabels, departments])}
			<p>loading additional metadata...</p>
		{:then [places, labels, filteredLabels, departments]}
			<div class="grid grid-area gap-4">
				<section class="actions flex flex-row gap-2">
					<button
						aria-label="save"
						onclick={() => {
							if (edited && thing.value != null) {
								saving.thing = true;
								updateThing(thing.value)
									.then(() => (thing.original = thing.value))
									.catch((err) => console.error('cannot save', err))
									.then(() => (saving.thing = false));
							}
						}}
						class={[
							edited && !saving.thing ? 'bg-green-800 hover:bg-green-700' : 'bg-gray-600',
							'rounded-md p-2'
						]}
						disabled={!edited || saving.thing}>{saving.thing ? 'Saving...' : 'Save'}</button
					>

					<label class="btn bg-green-800 hover:bg-green-700">
						Add image
						<input type="file" accept="image/*" hidden multiple onchange={saveImages} />
					</label>
				</section>
				<section class="images flex gap-2 flex-nowrap">
					{#each thing.value.image_ids as image_id (image_id)}
						<div class="h-40 w-40 bg-gray-600 p-2">
							<img
								class="max-h-36 max-w-36"
								src="/api/v1/things/{thing.id}/images/{image_id}"
								alt={image_id}
							/>
						</div>
					{/each}
				</section>
				<section class="main flex flex-col justify-stretch">
					<input type="text" name="name" bind:value={thing.value.name} />
					<textarea rows="7" name="description" bind:value={thing.value.description}></textarea>
				</section>
				<section class="side flex flex-col flex-nowrap gap-4">
					<div
						class="border-b-1 border-gray-600 grid grid-cols-2 grid-cols-[1fr_auto] gap-4 pb-4 px-2"
					>
						<p class="col-span-2 font-bold">data</p>

						<p>Amount</p>
						<input type="number" name="count" bind:value={thing.value.count} min="1" />
						<p>Location</p>
						<select bind:value={thing.value.in_place}>
							<option value={undefined}></option>
							{#each Object.entries(places) as [str_id, place] (str_id)}
								<option value={place.id}>{place.name}</option>
							{/each}
						</select>
						<p>Department</p>
						<p>
							{thing.value.department == undefined
								? 'None'
								: departments[thing.value.department].name}
						</p>
					</div>

					<div class="flex flex-col border-b-1 border-gray-600 pb-4">
						<button
							type="button"
							class="flex justify-between items-center p-2 rounded-md mb-4 hover:bg-gray-600"
							onclick={() => (editLabels = !editLabels)}
						>
							<p class="font-bold">Labels</p>
							<span class="icon-[material-symbols--edit-note]"></span>
						</button>

						<div class="relative">
							<div class="flex flex-wrap gap-2 px-2">
								{#each thing.value.label_ids as label_id, i (i)}
									{@const label_color = labels[label_id].color || 'gray'}
									<p
										class="px-2 py-1 border rounded-full font-bold text-sm flex items-center gap-1 size-max"
										style:border-color={label_color}
										style:color="color-mix(in srgb, {label_color} 50%, white)"
										style:background-color="color-mix(in srgb, {label_color} 25%, transparent)"
									>
										<span>{labels[label_id].name}</span>
										<button
											aria-label="delete"
											onclick={() => thing.value!.label_ids.splice(i, 1)}
											class="contents"
											><span class="icon-[material-symbols--close] text-[16px]"></span></button
										>
									</p>
								{:else}
									<p>No labels</p>
								{/each}
							</div>
							<div
								class="absolute top-0 left-0 right-0 bg-primary-900 border border-primary-600 rounded-lg p-4 flex flex-col gap-4 z-10"
								hidden={!editLabels}
							>
								<p>Apply labels to this thing</p>
								<input type="text" bind:value={labelSearch} />
								<div class="border-t-1 border-gray-600 flex flex-col">
									{#each filteredLabels as id (id)}
										{@const label = labels[id]}
										<button
											class="flex items-start border-b-1 border-gray-600 hover:bg-gray-600 py-2 gap-2 text-left"
											onclick={() => toggleLabel(thing.value!, id)}
										>
											<input
												type="checkbox"
												placeholder="Filter labels"
												checked={thing.value.label_ids.indexOf(id) != -1}
											/>
											<p style:background-color={label.color} class="w-4 h-4 rounded-full"></p>
											<div>
												<p class="text-base/4 mb-1">{label.name}</p>
												<p>{label.description}</p>
											</div>
										</button>
									{/each}
									{#if labelSearch != ''}
										<button
											class="py-2 hover:bg-gray-600"
											onclick={() =>
												newLabel(labelSearch).then((label) => {
													thing.value!.label_ids.push(label.id);
													labelSearch = '';
												})}>Create new label '{labelSearch}'</button
										>
									{/if}
								</div>
							</div>
						</div>
					</div>

					<div class="flex flex-col border-b-1 border-gray-600 pb-4">
						<button
							type="button"
							class="flex justify-between items-center p-2 rounded-md mb-4 hover:bg-gray-600"
						>
							<p class="font-bold">Reservations</p>
							<span class="icon-[material-symbols--edit-note]"></span>
						</button>

						<div class="grid grid-cols-3 grid-cols-[min-content_1fr_min_content] px-2">
							{#each thing.value.reservations as reservation (reservation.id)}~
								<p>x{reservation.count}</p>
								<p>{reservation.reserved_by}</p>
							{:else}
								<p class="col-span-2">No reservations</p>
							{/each}
						</div>
					</div>
				</section>
			</div>
		{:catch err}
			<p>loading failed: {err}</p>
		{/await}
	{/if}
</section>

<style lang="scss">
	section.actions {
		grid-area: act;
	}
	section.images {
		grid-area: img;
	}
	section.main {
		grid-area: main;
	}
	section.side {
		grid-area: side;
	}
	.grid-area {
		grid-template-areas:
			'act act'
			'img img'
			'main side';
		grid-template-columns: 1fr min-content;
	}
</style>

<script lang="ts">
	import { things, places, labels, type Thing, type Place } from '$lib/data.svelte';
	import { type Department } from '$lib/data/department.svelte';
	import { departments } from '$lib/data/department.svelte';
	import { getTabs, getActiveTab, setActiveTab } from './state.svelte';
	import { auth } from '$lib/auth.svelte';
	import { resolve } from '$app/paths';
	import { goto } from '$app/navigation';

	import DepartmentTab from './departmentTab.svelte';
	import Tabs from '$lib/components/Tabs.svelte';

	// const things = fetchThings();
	// const places = fetchPlaces();
	// const labels = fetchLabels().then(labels => labels.reduce((result, label) => {
	// 	result[label.id] = label.name;
	// 	return result;
	// }, {} as Record<number, string>));

	let displayMode = 'things' as 'department' | 'place' | 'things';

	const getDepartment = (
		thing: Thing | Place,
		places: Record<number, Place>,
		departments: Record<number, Department>
	): Department | undefined => {
		let thing_place = thing.in_place;
		while (thing_place != undefined) {
			const place = places[thing_place];
			const dep = place.in_department;
			if (dep != undefined) {
				return departments[dep];
			}
			thing_place = place.in_place;
		}
		return undefined;
	};

	let activeTab = $derived(getActiveTab());
</script>

{#if !auth}
	<p>Loading...</p>
{:else}
	<section class="flex flex-col gap-4 p-4">
		{#await Promise.all([things, places, departments, labels, auth])}
			<p>loading things...</p>
		{:then [things, places, departments, labels, auth]}
			<section>
				<Tabs tabs={getTabs()} {setActiveTab}></Tabs>
			</section>

			{#if activeTab.label == 'Departments'}
				<DepartmentTab {departments} />
			{:else if activeTab.label == 'Locations'}
				<section>
					<button
						type="button"
						class="rounded-md bg-green-800 p-2 hover:bg-green-700"
						onclick={() => goto(resolve('/inventory/places/new'))}>New place</button
					>
				</section>

				<section class="grid grid-cols-[max-content_1fr_repeat(3,max-content)] gap-x-4 gap-y-2">
					<p>image</p>
					<p>name</p>
					<p>location</p>
					<p>department</p>
					<p>reserved</p>

					{#each Object.entries(places) as [id, place] (id)}
						{@const dep = getDepartment(place, places, departments)}
						<div class="flex h-20 w-20 items-center justify-center bg-gray-600 p-2">
							{#if place.main_img}
								<img
									class="max-h-16 max-w-16"
									src="/api/v1/places/{place.id}/images/{place.main_img}"
									alt=""
								/>
							{:else}
								<span
									class="icon-[material-symbols--no-photography-outline] bg-gray-200 text-[24px]"
								></span>
							{/if}
						</div>

						<p>{place.name}</p>
						<div>
							{#if place.in_place == undefined}
								<span></span>
							{:else}
								<span>{places[place.in_place].name}</span>
							{/if}
						</div>
						<div>
							{#if dep == undefined}
								<span></span>
							{:else}
								<span>{dep.name}</span>
							{/if}
						</div>
						<div>
							{#if place.reservations.length == 0}
								<p></p>
							{:else}
								{#each place.reservations as reservation (reservation.id)}
									<p>{reservation.reserved_by}</p>
								{/each}
							{/if}
						</div>
					{/each}
				</section>
			{:else if activeTab.label == 'Things'}
				<section>
					<button
						type="button"
						class="rounded-md bg-green-800 p-2 hover:bg-green-700"
						onclick={() => goto(resolve('/inventory/things/new'))}>New item</button
					>
				</section>

				<section class="grid grid-cols-[max-content_1fr_repeat(6,max-content)] gap-x-4 gap-y-2">
					<p>image</p>
					<p>name</p>
					<p>amount</p>
					<p>location</p>
					<p>department</p>
					<p>labels</p>
					<p>reserved</p>
					<p>actions</p>

					{#each things as thing (thing.id)}
						{@const dep = getDepartment(thing, places, departments)}
						<a
							href={resolve(`/inventory/things/${thing.id}`)}
							class="flex h-20 w-20 items-center justify-center bg-gray-600 p-2"
						>
							{#if thing.main_img}
								<img
									class="max-h-16 max-w-16"
									src="/api/v1/things/{thing.id}/images/{thing.main_img}"
									alt=""
								/>
							{:else}
								<span
									class="icon-[material-symbols--no-photography-outline] bg-gray-200 text-[24px]"
								></span>
							{/if}
						</a>

						<a href={resolve(`/inventory/things/${thing.id}`)}>{thing.name}</a>
						<p>{thing.count - thing.reservations.map((r) => r.count).reduce((a, b) => a + b, 0)}</p>
						<div>
							{#if thing.in_place == undefined}
								<span></span>
							{:else}
								<span>{places[thing.in_place].name}</span>
							{/if}
						</div>
						<div>
							{#if dep == undefined}
								<span></span>
							{:else}
								<span>{dep.name}</span>
							{/if}
						</div>
						<div>
							{#if thing.label_ids.length == 0}
								<p></p>
							{:else}
								{#each thing.label_ids as label_id (label_id)}
									{@const label = labels[label_id]}
									{@const label_color = label.color || 'gray'}
									<p
										class="flex size-max items-center rounded-full border px-2 py-1 text-sm font-bold"
										style:border-color={label_color}
										style:color="color-mix(in srgb, {label_color} 50%, white)"
										style:background-color="color-mix(in srgb, {label_color} 25%, transparent)"
									>
										{labels[label_id].name}
									</p>
								{/each}
							{/if}
						</div>
						<div>
							{#if thing.reservations.length == 0}
								<p></p>
							{:else}
								{#each thing.reservations as reservation (reservation.id)}
									<p>{reservation.reserved_by} ({reservation.count})</p>
								{/each}
							{/if}
						</div>
						<div class="flex items-center gap-2">
							{#if auth.level == 'REQUESTER' || auth.level == 'EDITOR'}
								<button type="button" class="rounded-md bg-green-800 p-2 hover:bg-green-700"
									>reserve</button
								>
							{/if}
							<a
								class="rounded-md bg-green-800 p-2 hover:bg-green-700"
								href={resolve(`/inventory/things/${thing.id}`)}
							>
								{#if auth.level == 'EDITOR'}
									<span class="icon-[material-symbols--edit]"></span>
									<span>edit</span>
								{:else}
									<span class="icon-[material-symbols--edit] text-[24px]"></span>
									<span>view</span>
								{/if}
							</a>
						</div>
					{/each}
				</section>
			{/if}
		{:catch err}
			<p>loading failed: {err}</p>
		{/await}
	</section>
{/if}

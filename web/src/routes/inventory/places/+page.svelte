<script lang="ts">
	import { deletePlace, type Place } from '$lib/data/places.svelte';
	import type { Department } from '$lib/data/department.svelte';
	import SubPlaceRow from './subPlaceRow.svelte';
	import Input from '$lib/components/Input.svelte';
	import Select from '$lib/components/Select.svelte';
	import Button from '$lib/components/Button.svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { canEdit } from '$lib/auth.svelte';
	import { tick } from 'svelte';

	let {
		places,
		departments
	}: { places: Record<number, Place>; departments: Record<number, Department> } = $props();

	let placeFilter = $state<string>('');
	let departmentFilter = $state<string>('');
	const reserveFilterOptions = ['All', 'Reserved', 'Not Reserved'];
	let reservedFilter = $state<(typeof reserveFilterOptions)[number]>(reserveFilterOptions[0]);

	const filteredPlaces = $derived(
		Object.fromEntries(
			Object.entries(places).filter(
				([key, place]) => checkPlace(place) && checkReserved(place) && checkDepartment(place)
			)
		)
	);

	const checkReserved = (place: Place) =>
		reservedFilter === 'All' ||
		(reservedFilter === 'Reserved' && place.reservations.length > 0) ||
		(reservedFilter === 'Not Reserved' && place.reservations.length === 0);
	const checkDepartment = (place: Place) =>
		departmentFilter === '' ||
		(place.in_department != undefined &&
			departments[place.in_department]?.name
				.toLowerCase()
				.includes(departmentFilter.toLowerCase()));
	const checkPlace = (place: Place) =>
		placeFilter === '' || place.name.toLowerCase().includes(placeFilter.toLowerCase());

	let openedRow = $state<string | null>(null);
	function openRow(key: string) {
		openedRow = openedRow === key ? null : key;
		// tick().then(() => {
		// 	if (openedRow === key)
		// 		document
		// 			.getElementById(`expanded-row-${key}`)
		// 			?.scrollIntoView({ behavior: 'smooth', block: 'start', inline: 'nearest' });
		// });
		setTimeout(() => {
			if (openedRow === key)
				document
					.getElementById(`expanded-row-${key}`)
					?.scrollIntoView({ behavior: 'smooth', block: 'start', inline: 'nearest' });
		}, 50);
	}
</script>

<section>
	<div class="filters flex justify-end space-x-4">
		<Input
			bind:value={placeFilter}
			placeholder="Filter by place name"
			className="w-64"
			label="Place Name"
		/>
		<Input
			bind:value={departmentFilter}
			placeholder="Filter by department name"
			className="w-64"
			label="Department Name"
		/>

		<Select
			bind:value={reservedFilter}
			options={reserveFilterOptions}
			className="w-64"
			label="Reserved Status"
		/>
	</div>

	<table class="min-w-full table-auto border-collapse">
		<thead>
			<tr class="border-b border-gray-300">
				<th class="w-fit p-2 text-left">Image</th>
				<th class="w-1/2 p-2 text-left">Name</th>
				<th class="w-1/2p-2 text-left">Description</th>
				<th class="w-fit p-2 text-left">Contains</th>
				<th class="w-fit p-2 text-left whitespace-nowrap">Resides in</th>
				<th class="w-fit p-2 text-left">Department</th>
				<th class="w-fit p-2 text-left">Reserved</th>
				<th class="w-fit whitespace-nowrap"
					><Button
						icon="plus"
						label="Create place"
						class="my-1 whitespace-nowrap"
						onClick={() => goto(resolve(`/inventory/places/new`))}
					></Button>
				</th>
			</tr>
		</thead>
		<tbody>
			{#each Object.entries(filteredPlaces) as [key, place] (key)}
				<tr
					class="{place.description
						? 'cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700'
						: ''} border-gray-200 transition-colors duration-300 ease-in-out not-last:border-b"
					onclick={() => place.description && openRow(key)}
				>
					<td class="h-20 max-h-20 w-20 max-w-20 p-2">
						{#if place.main_img}
							<img class="max-h-16 max-w-16" src={place.main_url} alt="" />
						{:else}
							<div class="flex items-center justify-center">
								<span
									class="icon-[material-symbols--no-photography-outline] bg-gray-200 text-[20px]"
								></span>
							</div>
						{/if}
					</td>
					<td class="w-1/2 p-2 text-left">{place.name}</td>
					<td class="w-1/2 p-2 text-left whitespace-nowrap"
						>{#if place.description}
							{place.description.substring(0, 100)}
							{place.description.length > 100 ? '...' : ''}
						{/if}
					</td>
					<td class="w-fit p-2 text-left">
						<SubPlaceRow sub_place={place} key_id={place.id.toString()} />
					</td>
					<td class="w-fit p-2 text-left whitespace-nowrap"
						>{#if place.in_place}{places[place.in_place]?.name}{/if}</td
					>
					<td class="w-fit p-2 text-left"
						>{#if place.in_department}{departments[place.in_department]?.name ?? ''}{/if}</td
					>
					<td class="w-fit p-2 text-left">{place.reservations.length > 0 ? 'Yes' : 'No'}</td>
					{#if canEdit()}
						<td class="flex h-20 w-fit items-center gap-3 p-2">
							<Button
								icon="edit"
								label="Edit"
								iconOnly
								circle
								onClick={() => goto(resolve('/inventory/places/new') + `?id=${place.id}`)}
							/>
							<Button
								icon="delete"
								label="Delete"
								iconOnly
								circle
								status="danger"
								onClick={async () => {
									await deletePlace(place.id);
								}}
							/>
						</td>
					{/if}
				</tr>
				{#if openedRow === key}
					<tr class="bg-gray-100 dark:bg-gray-700" id="expanded-row-{key}">
						<td></td>
						<td colspan="6" class="p-2">
							<h2 class="text-lg font-bold">Description</h2>
							<p class="whitespace-pre-wrap">{place.description}</p>
							<h2 class="text-lg font-bold">Images</h2>
							{#if place.image_ids.length > 0 && place.image_urls}
								<div class="flex flex-wrap gap-2">
									{#each place.image_ids as imageId}
										<img
											class="max-h-64 max-w-64 rounded border border-gray-300 object-cover"
											src={place.image_urls[imageId]}
											alt=""
										/>
									{/each}
								</div>
							{:else}
								<p>No images available.</p>
							{/if}
						</td>
						<td></td>
					</tr>
				{/if}
			{/each}
		</tbody>
	</table>
</section>

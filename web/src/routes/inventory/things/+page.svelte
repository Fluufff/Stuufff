<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { observeListElement } from '$lib/actions/listObserver';
	import Button from '$lib/components/Button.svelte';
	import Image from '$lib/components/Image.svelte';
	import Input from '$lib/components/Input.svelte';
	import Select from '$lib/components/Select.svelte';
	import type { Department } from '$lib/data/department.svelte';
	import type { Label } from '$lib/data/label.svelte';
	import type { Place } from '$lib/data/places.svelte';
	import {
		fetchThingExpanded,
		isThingExpanded,
		type Thing,
		type ThingExpanded
	} from '$lib/data/things.svelte';
	import type { SvelteMap } from 'svelte/reactivity';

	let {
		things,
		places,
		labels,
		departments
	}: {
		things: Record<number, Thing | ThingExpanded>;
		labels: SvelteMap<number, Label>;
		places: Record<number, Place>;
		departments: Record<number, Department>;
	} = $props();

	let nameFilter = $state('');
	let placeFilter = $state('');
	let departmentFilter = $state('');
	const availabilityStatusses = ['All', 'Unavailable', 'Available'];
	let availabilityStatus = $state<(typeof availabilityStatusses)[number]>(availabilityStatusses[0]);
	let selectedLabelIDs = $state<string[]>([]);

	const filteredThings = $derived(
		Object.fromEntries(
			Object.entries(things).filter(
				([_, thing]) =>
					checkName(thing) &&
					checkPlace(thing) &&
					checkDepartment(thing) &&
					checkAvailability(thing) &&
					checkLabels(thing)
			)
		)
	);
	function checkName(thing: Thing) {
		return thing.name.toLocaleLowerCase().includes(nameFilter.toLocaleLowerCase());
	}
	function checkPlace(thing: Thing) {
		if (!placeFilter.trim()) return true;
		if (!thing.in_place) return false;
		const foundPlaces = Object.fromEntries(
			Object.entries(places).filter(([_, place]) =>
				place.name.toLocaleLowerCase().includes(placeFilter.toLocaleLowerCase())
			)
		);
		return !!foundPlaces[thing.in_place];
	}
	function checkDepartment(thing: Thing | ThingExpanded) {
		if (!departmentFilter.trim()) return true;
		if (!isThingExpanded(thing)) return false;
		if (!thing.department) return false;
		const foundDepartments = Object.fromEntries(
			Object.entries(departments).filter(([_, dep]) =>
				dep.name.toLocaleLowerCase().includes(placeFilter.toLocaleLowerCase())
			)
		);
		return !!foundDepartments[thing.department];
	}
	function checkAvailability(thing: Thing) {
		switch (availabilityStatus) {
			case 'All':
				return true;
			case 'Unavailable':
				return thing.count - thing.reservations.length <= 0;
			case 'Available':
				return thing.count - thing.reservations.length > 0;
		}
	}
	function checkLabels(thing: Thing) {
		if (!selectedLabelIDs.length) return true;
		const castLabels = selectedLabelIDs.map((l) => Number(l));
		return castLabels.every((l) => thing.label_ids.includes(l));
	}

	function getPlaceOfThing(placeID: number) {
		return places[placeID];
	}
	function getDepartmentOfThing(depID: number) {
		return departments[depID];
	}

	function getThingLabels(thing: Thing | ThingExpanded) {
		return [...thing.label_ids].map((labelID) => labels.get(labelID)).map((label) => label) ?? [];
	}
</script>

<section>
	<div class="filters flex items-center justify-end gap-4">
		<Input
			bind:value={nameFilter}
			placeholder="Filter by name"
			className="w-64"
			label="Thing name"
		/>
		<Input
			bind:value={placeFilter}
			placeholder="Filter by place"
			className="w-64"
			label="Place name"
		/>
		<Input
			bind:value={departmentFilter}
			placeholder="Filter by department"
			className="w-64"
			label="Department name"
		/>
		<Select
			bind:value={availabilityStatus}
			options={availabilityStatusses}
			className="w-64"
			label="Availability"
		/>
		<Select
			bind:value={selectedLabelIDs}
			options={[
				...labels.values().map((label) => ({ value: String(label.id), label: label.name }))
			]}
			className="w-64"
			label={`Labels (${selectedLabelIDs.length} selected)`}
			multiple
			noPills
		/>
	</div>
	<table class="min-w-full table-auto border-collapse">
		<thead>
			<tr class="border-b border-gray-300">
				<th class="w-fit p-2 text-left">Image</th>
				<th class="w-1/2 p-2 text-left">Name</th>
				<th class="w-1/2p-2 text-left">Description</th>
				<th class="w-fit p-2 text-center">Count</th>
				<th class="w-fit p-2 text-left whitespace-nowrap">Resides in</th>
				<th class="w-fit p-2 text-left">Department</th>
				<th class="w-fit p-2 text-center">Reserved</th>
				<th class="w-fit whitespace-nowrap"
					><Button
						icon="plus"
						label="Create thing"
						class="my-1 whitespace-nowrap"
						onClick={() => goto(resolve(`/inventory/things/new`))}
					></Button>
				</th>
			</tr>
		</thead>
		<tbody id="thingsListRoot">
			{#each Object.entries(filteredThings) as [key, thing] (key)}
				<tr
					use:observeListElement={{
						callback: () => fetchThingExpanded(thing.id),
						rootElement: 'thingsListRoot',
						once: true
					}}
				>
					<td colspan="8">
						<div class="my-1 flex w-fit items-center gap-3 text-sm">
							{#each getThingLabels(thing) as label (label?.id)}
								{#if label}
									<span class="rounded px-3 py-1.5" style="background-color: {label.color}"
										>{label.name}</span
									>
								{/if}
							{/each}
						</div>
					</td>
				</tr>
				<tr class="border-gray-200 transition-colors duration-300 ease-in-out not-last:border-b">
					<td><Image url={thing.main_url} alt="Image of {thing.name}" /></td>
					<td>{thing.name}</td>
					<td>
						{#if thing.description}
							{thing.description.substring(0, 100)}
							{thing.description.length > 100 ? '...' : ''}
						{/if}
					</td>
					<td class="text-center">{thing.count}</td>
					<td>
						{#if thing.in_place}
							{getPlaceOfThing(thing.in_place)?.name}{/if}
					</td>
					<td>
						{#if isThingExpanded(thing)}
							{#if thing.department}
								{getDepartmentOfThing(thing.department)?.name}{/if}
						{/if}
					</td>
					<td class="text-center">{thing.reservations.length} / {thing.count}</td>
					<td></td>
				</tr>
			{/each}
		</tbody>
	</table>
</section>

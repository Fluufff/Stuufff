<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';
	import type { Place } from '$lib/data/places.svelte';
	import SubPlaceRow from './subPlaceRow.svelte';

	let { sub_place, key_id }: { sub_place: Place; key_id: string } = $props();

	let isOpen = $state(false);
</script>

{#if sub_place.sub_places && sub_place.sub_places.length > 0}
	<button
		onclick={() => (isOpen = !isOpen)}
		class="flex cursor-pointer items-center justify-center space-x-2"
	>
		<div class="h-6 w-6 border-b border-l border-gray-500"></div>
		<p class="whitespace-nowrap">{sub_place.name}</p>
		<Icon
			name="chevron_right"
			class="transition-transform duration-300 ease-in-out {isOpen ? 'rotate-90' : ''}"
		/>
	</button>
{:else}
	<div class="flex space-x-2">
		<div class="h-6 w-6 border-b border-l border-gray-500"></div>
		<p class="whitespace-nowrap">{sub_place.name}</p>
	</div>
{/if}
{#if isOpen && sub_place.sub_places}
	<div class="ml-3">
		{#each sub_place.sub_places as sub_sub_place (key_id + '-' + sub_sub_place.id)}
			<SubPlaceRow sub_place={sub_sub_place} key_id={key_id + '-' + sub_sub_place.id} />
		{/each}
	</div>
{/if}

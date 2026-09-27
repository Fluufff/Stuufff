<script lang="ts">
	import { onMount } from 'svelte';
	import type { Department } from '$lib/data';

	let { departments }: { departments: Record<number, Department> } = $props();

	onMount(() => {
		console.log('Department tab mounted', departments);
	});
</script>

<section class="grid grid-cols-[max-content_1fr] gap-x-4 gap-y-2">
	<p>image</p>
	<p>name</p>

	{#each Object.entries(departments) as [id, department] (id)}
		<div class="flex h-20 w-20 items-center justify-center bg-gray-600 p-2">
			{#if department.main_img}
				<img
					class="max-h-16 max-w-16"
					src="/api/v1/places/{department.id}/images/{department.main_img}"
					alt=""
				/>
			{:else}
				<span class="icon-[material-symbols--no-photography-outline] bg-gray-200 text-[24px]"
				></span>
			{/if}
		</div>

		<p>{department.name}</p>
	{/each}
</section>

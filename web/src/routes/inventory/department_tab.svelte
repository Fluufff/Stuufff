<script lang="ts">
	import { onMount } from 'svelte';
	import type { Department } from '$lib/data';

	let { departments }: { departments: Record<number, Department> } = $props();

	onMount(() => {
		console.log('Department tab mounted', departments);
	});
</script>

<table class="min-w-full table-auto border-collapse">
	<thead>
		<tr class="border-b border-gray-300">
			<th class="w-fit p-2 text-left">Image</th>
			<th class=" p-2 text-left">Name</th>
		</tr>
	</thead>
	<tbody>
		{#each Object.entries(departments) as [id, department] (id)}
			<tr
				class="cursor-pointer border-gray-200 transition-colors duration-300 ease-in-out not-last:border-b hover:bg-gray-100"
			>
				<td class="h-20 max-h-20 w-20 max-w-20 p-2">
					{#if department.main_img}
						<img
							class="max-h-16 max-w-16"
							src="/api/v1/places/{department.id}/images/{department.main_img}"
							alt=""
						/>
					{:else}
						<div class="flex items-center justify-center">
							<span class="icon-[material-symbols--no-photography-outline] bg-gray-200 text-[24px]"
							></span>
						</div>
					{/if}
				</td>
				<td class="p-2">{department.name}</td>
			</tr>
		{/each}
	</tbody>
</table>

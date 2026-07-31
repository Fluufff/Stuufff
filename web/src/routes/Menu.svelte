<script lang="ts">
	import { page } from '$app/state';

	import { routes } from './routes';

	const activeClass = (pagePath: string, routePath: string) => {
		if (pagePath.startsWith(routePath)) {
			return 'bg-gray-200 dark:bg-gray-600';
		}
	};
</script>

<ul class="my-4 flex flex-col">
	{#each routes as route (route.path)}
		<li
			class="mb-2 px-4 py-3 hover:bg-gray-200 dark:hover:bg-gray-600 {activeClass(
				page.url.pathname,
				route.path
			)}"
		>
			<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
			<a href={route.path} class="flex h-[32px] items-center"
				>{#if route.icon}<span class="{route.icon} mr-2"></span>{/if}<span>{route.name}</span></a
			>
		</li>
	{/each}
</ul>

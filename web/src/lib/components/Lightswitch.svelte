<script lang="ts">
	import { Switch } from '@skeletonlabs/skeleton-svelte';

	let checked = $state(false);

	$effect(() => {
		const mode = localStorage.getItem('mode') || 'light';
		checked = mode === 'light';
	});

	const onCheckedChange = (event: { checked: boolean }) => {
		const mode = event.checked ? 'light' : 'dark';
		document.documentElement.setAttribute('data-mode', mode);
		localStorage.setItem('mode', mode);
		checked = event.checked;
	};
</script>

<svelte:head>
	<script>
		(() => {
			const mode = localStorage.getItem('mode') || 'light';
			document.documentElement.setAttribute('data-mode', mode);
		})();
	</script>
</svelte:head>

<Switch {checked} {onCheckedChange}>
	{#snippet inactiveChild()}
		<span class="icon-[material-symbols--dark-mode-outline] text-[12px]"></span>
	{/snippet}
	{#snippet activeChild()}
		<span class="icon-[material-symbols--light-mode-outline] text-[12px]"></span>
	{/snippet}
</Switch>

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
	<Switch.Control>
		<Switch.Thumb>
			<Switch.Context>
				{#snippet children(switch_)}
					{#if switch_().checked}
						<span class="icon-[material-symbols--light-mode-outline] text-[12px]"></span>
					{:else}
						<span class="icon-[material-symbols--dark-mode-outline] text-[12px]"></span>
					{/if}
				{/snippet}
			</Switch.Context>
		</Switch.Thumb>
	</Switch.Control>
	<Switch.HiddenInput />
</Switch>

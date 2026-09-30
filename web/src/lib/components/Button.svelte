<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';
	import type { icon } from '$lib/data/icons';
	import Tooltip from './Tooltip.svelte';
	let {
		icon: iconName,
		label,
		circle,
		iconOnly,
		type,
		fullWidth,
		status,
		onClick
	}: {
		icon?: icon;
		label?: string;
		circle?: boolean;
		iconOnly?: boolean;
		type?: 'button' | 'submit' | 'reset';
		fullWidth?: boolean;
		status?: 'danger' | 'warning' | 'success';
		onClick?: () => void;
	} = $props();

	let buttonID = `button-${Math.random().toString(36).substr(2, 9)}`;
</script>

<Tooltip title={label} target={buttonID} {fullWidth}>
	<button
		onclick={onClick}
		type={type ?? 'button'}
		class="flex cursor-pointer items-center gap-2 {circle ? 'rounded-full' : 'rounded'} {iconOnly
			? 'h-[32px] w-[32px] justify-center'
			: 'px-4 py-2'} {fullWidth ? 'w-full justify-center' : ''} border-1 {status === 'danger'
			? 'border-severity-danger hover:bg-severity-danger focus:bg-severity-danger'
			: status === 'warning'
				? 'border-severity-noncompliant hover:bg-severity-noncompliant focus:bg-severity-noncompliant'
				: status === 'success'
					? 'border-severity-compliant hover:bg-severity-compliant focus:bg-severity-compliant'
					: 'border-green-600 hover:bg-green-600 focus:bg-green-600'} transition-colors duration-300 ease-in-out"
	>
		{#if iconName}
			<Icon name={iconName} />
		{/if}
		{#if label && !iconOnly}
			{label}
		{/if}
	</button>
	<div id={buttonID} class="relative"></div>
</Tooltip>

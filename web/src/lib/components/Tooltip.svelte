<script lang="ts">
	import type { Snippet } from 'svelte';
	import { teleport } from '$lib/actions/teleport';

	let {
		target = 'teleport',
		title,
		fullWidth,
		children
	}: { target?: string; title?: string; fullWidth?: boolean; children: Snippet } = $props();

	let isHovered = $state(false);
	let tooltipElement = $state<HTMLDivElement>();
	let anchorX = $state<number>(0);
	let anchorY = $state<number>(0);
	let x = $state<number>(0);
	let y = $state<number>(0);

	$effect(() => {
		if (!isHovered || !tooltipElement) return;

		const { width, height } = tooltipElement.getBoundingClientRect();
		const gap = 12;
		let left = anchorX + gap;
		let top = anchorY + gap;

		if (left + width > window.innerWidth - gap) left = anchorX - width - gap;
		if (top + height > window.innerHeight - gap) top = anchorY - height - gap;

		x = Math.max(gap, Math.min(left, window.innerWidth - width - gap));
		y = Math.max(gap, Math.min(top, window.innerHeight - height - gap));
	});

	function mouseOver(event: MouseEvent | FocusEvent) {
		isHovered = true;

		if (event instanceof MouseEvent) {
			anchorX = event.clientX;
			anchorY = event.clientY;
		} else {
			const targetSize = (event.target as HTMLElement).getBoundingClientRect();
			anchorX = targetSize.left + targetSize.width / 2;
			anchorY = targetSize.bottom;
		}
	}
	function mouseMove(event: MouseEvent) {
		anchorX = event.clientX;
		anchorY = event.clientY;
	}
	function mouseLeave() {
		isHovered = false;
	}
</script>

<div
	onmouseover={mouseOver}
	onmouseleave={mouseLeave}
	onmousemove={mouseMove}
	onfocus={mouseOver}
	onblur={mouseLeave}
	role="tooltip"
	class={fullWidth ? 'w-full' : 'w-fit'}
>
	{@render children()}
</div>

{#if isHovered}
	<div
		bind:this={tooltipElement}
		use:teleport={target}
		style="top: {y}px; left: {x}px;"
		class="tooltip pointer-events-none fixed z-50 max-w-[calc(100vw-1rem)] rounded border border-gray-300 bg-white px-4 py-2 whitespace-normal shadow-md"
	>
		{title}
	</div>
{/if}

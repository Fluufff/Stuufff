<script lang="ts">
	let {
		url,
		alt,
		class: classes = 'max-h-16 max-w-16 object-contain',
		size = 64
	}: { url?: string | null; alt?: string; class?: string; size?: number } = $props();

	let imageID = `image-${Math.random().toString(36).slice(2, 11)}`;
</script>

{#if url}
	<img
		id={imageID}
		class={classes}
		src={url}
		{alt}
		width={size}
		height={size}
		loading="lazy"
		decoding="async"
	/>
{:else}
	<div class="flex items-center justify-center {classes}">
		<span class="icon-[material-symbols--no-photography-outline] bg-gray-200 text-[20px]"></span>
	</div>
{/if}

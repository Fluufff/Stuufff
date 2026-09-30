<script lang="ts">
	import { icon } from '$lib/data/icons';

	let {
		name,
		size = 16,
		class: className = ''
	}: { name: keyof typeof icon; size?: number; class?: string } = $props();
</script>

<i class={icon[name] + ` text-[${size}px] ` + className}></i>

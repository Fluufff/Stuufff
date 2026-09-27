<script lang="ts">
	type Tab = {
		label?: string;
		active: boolean;
		icon?: string;
		id: number;
	};

	let { tabs, setActiveTab }: { tabs: Tab[]; setActiveTab: (id: number) => void } = $props();
</script>

<div class="tabs flex w-fit overflow-hidden">
	{#each tabs as tab, index (tab.id)}
		<button
			class="tab flex gap-2 border-1 border-green-600 px-4 py-2 transition-all duration-300 ease-in-out {tab.active
				? 'active cursor-default bg-green-800 text-white'
				: 'cursor-pointer hover:bg-green-600'} {index == tabs.length - 1
				? 'rounded-r-md'
				: ''} {index == 0 ? 'rounded-l-md' : ''}"
			onclick={() => setActiveTab(tab.id)}
		>
			{#if tab.icon}
				<img src={tab.icon} alt={tab.label} />
			{/if}
			{#if tab.label}
				<span>{tab.label}</span>
			{/if}
		</button>
	{/each}
</div>

<style></style>

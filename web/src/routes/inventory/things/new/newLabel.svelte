<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import Input from '$lib/components/Input.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import Textarea from '$lib/components/Textarea.svelte';
	import { createLabel } from '$lib/data/label.svelte';

	let { open = $bindable() }: { open: boolean } = $props();

	let name = $state('');
	let color = $state('#f7c3a1');
	let description = $state('');

	let saving = $state(false);
	let nameValid = $state(true);
	let colorValid = $state(true);
	let cansave = $derived(nameValid && colorValid);
	let error = $state<string | null>(null);

	async function save() {
		saving = true;
		error = null;
		try {
			await createLabel({ name, color, description });
			open = false;
		} catch (err) {
			console.error('cannot save', err);
			error = 'Failed to save label';
		} finally {
			saving = false;
		}
	}
</script>

<Modal title="New Label" {open}>
	<Input label="Label name" bind:value={name} bind:valid={nameValid} required />
	<Input label="Label color" bind:value={color} bind:valid={colorValid} type="color" required />
	<Textarea label="Label description" bind:value={description} rows={3} />

	{#snippet footer()}
		<div class="flex space-x-3">
			<Button label="Cancel" onClick={() => (open = false)} fullWidth />
			<Button
				fullWidth
				onClick={save}
				disabled={!cansave || saving}
				label={saving ? 'Saving...' : 'Save'}
				status="success"
			/>
		</div>
		{#if error}
			<p class="text-red-500">{error}</p>
		{/if}
	{/snippet}
</Modal>

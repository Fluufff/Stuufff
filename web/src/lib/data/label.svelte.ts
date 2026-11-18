import { SvelteMap } from 'svelte/reactivity';

export interface Label {
	id: number;
	name: string;
	description?: string;
	color?: string;
}

export const labels = $state<SvelteMap<number, Label>>(await fetchLabels());

async function fetchLabels() {
	const response = await fetch(`/api/v1/labels`);
	if (!response.ok) {
		throw new Error('Failed to fetch labels');
	}

	const labelsArray: Label[] = await response.json();
	const labelsMap: SvelteMap<number, Label> = new SvelteMap();
	for (const label of labelsArray) {
		labelsMap.set(label.id, label);
	}
	return labelsMap;
}

export async function createLabel(label: Omit<Label, 'id'>): Promise<Label> {
	const response = await fetch('/api/v1/labels', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(label)
	});

	if (!response.ok) {
		throw new Error('Failed to create label');
	}

	const newLabel: Label = await response.json();
	labels.set(newLabel.id, newLabel);

	return newLabel;
}

export function findLabel(id: number): Label | undefined {
	return labels.get(id);
}

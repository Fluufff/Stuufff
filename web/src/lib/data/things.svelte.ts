import type { Reservation } from '$lib/data/reservations.svelte';

export interface Thing {
	id: number;
	name: string;
	count: number;
	description: string | null;
	in_place?: number | null;
	main_img: string | null;
	main_url?: string;
	reservations: Reservation[];
	label_ids: number[];
}

export interface ThingExpanded extends Thing {
	department: number | null;
	image_ids: string[];
	image_urls?: Record<string, string>;
}

type ThingResponse = Omit<Thing, 'main_url'>;
type ThingExpandedResponse = Omit<ThingExpanded, 'main_url' | 'image_urls'>;

export const things = $state<Record<number, Thing | ThingExpanded>>(
	await fetchThings().then((things) => things)
);

async function fetchThings() {
	const resp = await fetch(`/api/v1/things`);
	if (!resp.ok) {
		throw new Error('Failed to fetch things');
	}

	const thingResponses: ThingResponse[] = await resp.json();

	return thingResponses.reduce(
		(res, thing) => {
			res[thing.id] = addMainImageUrl(thing);
			return res;
		},
		{} as Record<number, Thing>
	);
}

export async function fetchThingExpanded(id: number): Promise<ThingExpanded> {
	const resp = await fetch(`/api/v1/things/${id}`);
	if (!resp.ok) {
		throw new Error('Failed to fetch thing');
	}

	const thingExpandedResponse: ThingExpandedResponse = await resp.json();
	const thingExpanded = addImageUrls(thingExpandedResponse);
	things[thingExpanded.id] = thingExpanded;
	return thingExpanded;
}

function addMainImageUrl(thing: ThingResponse): Thing {
	return {
		...thing,
		main_url: thing.main_img ? `/api/v1/things/${thing.id}/images/${thing.main_img}` : undefined
	};
}

function addImageUrls(thing: ThingExpandedResponse): ThingExpanded {
	const image_urls = Object.fromEntries(
		thing.image_ids.map((id) => [id, `/api/v1/things/${thing.id}/images/${id}`])
	);

	return {
		...thing,
		image_urls,
		main_url: thing.main_img ? image_urls[thing.main_img] : undefined
	} as ThingExpanded;
}

export async function createThing(
	thing: Omit<
		ThingExpanded,
		'id' | 'main_url' | 'image_urls' | 'image_ids' | 'main_img' | 'reservations'
	>
): Promise<Thing> {
	const response = await fetch('/api/v1/things', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(thing)
	});

	if (!response.ok) {
		throw new Error('Failed to create thing');
	}

	const { id } = await response.json();
	things[id] = {
		...thing,
		reservations: [] as Reservation[],
		main_img: null,
		id
	} as Thing;

	await updateThing({ ...thing, ...things[id] } as ThingExpanded);
	let expandedThing = await fetchThingExpanded(id);
	things[id] = {
		...things[id],
		...expandedThing
	};
	return things[id];
}

export async function updateThing(thing: ThingExpanded): Promise<void> {
	const response = await fetch(`/api/v1/things/${thing.id}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(thing)
	});

	if (!response.ok) {
		throw new Error('Failed to update thing');
	}

	things[thing.id] = thing;
}

export async function uploadImage(thingId: number, file: File): Promise<void> {
	const response = await fetch(`/api/v1/things/${thingId}/images`, {
		method: 'POST',
		headers: {
			'Content-Type': file.type
		},
		body: file
	});

	if (!response.ok) {
		throw new Error('Failed to upload thing image');
	}
}

export function isThingExpanded(thing: Thing | ThingExpanded): thing is ThingExpanded {
	return 'image_ids' in thing && 'department' in thing;
}

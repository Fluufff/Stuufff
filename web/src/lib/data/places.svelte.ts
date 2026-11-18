import { FetchError } from '$lib/data.svelte';
import type { Reservation } from '$lib/data/reservations.svelte';

export interface Place {
	id: number;
	name: string;
	description: string;
	main_img?: string | null;
	main_url?: string | null;
	image_ids: string[];
	image_urls?: Record<string, string>;
	in_department?: number;
	in_place?: number;
	sub_places?: Place[];
	reservations: Reservation[];
}

type PlaceResponse = Omit<Place, 'main_url' | 'image_urls'>;

export const places = $state<Record<number, Place>>(await fetchPlaces().then((places) => places));

async function fetchPlaces() {
	const resp = await fetch(`/api/v1/places`);
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to fetch places');
	}

	const placeResponses: PlaceResponse[] = await resp.json();

	const places = placeResponses.map(addImageUrls);

	const recursivedPlaces = places.reduce(
		(res, place) => {
			const childPlaces = places.filter((p) => p.in_place === place.id);
			if (childPlaces.length > 0) {
				place.sub_places = childPlaces;
			}
			res[place.id] = place;
			return res;
		},
		{} as Record<number, Place>
	);
	return recursivedPlaces;
}

function addImageUrls(place: PlaceResponse): Place {
	const image_urls = Object.fromEntries(
		place.image_ids.map((id) => [id, `/api/v1/places/${place.id}/images/${id}`])
	);

	return {
		...place,
		image_urls,
		main_url: place.main_img ? image_urls[place.main_img] : undefined
	} as Place;
}

export async function createPlace(
	place: Omit<Place, 'id' | 'main_url' | 'image_urls'>
): Promise<Place> {
	const resp = await fetch(`/api/v1/places`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(place)
	});
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to create place');
	}
	const { id } = await resp.json();
	return { ...place, id } as Place;
}

export async function uploadPlaceImage(placeId: number, file: File): Promise<void> {
	const resp = await fetch(`/api/v1/places/${placeId}/images`, {
		method: 'POST',
		headers: {
			'Content-Type': file.type
		},
		body: file
	});
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to upload place image');
	}
}

export async function updatePlace(place: Partial<Place> & { id: number }): Promise<Place> {
	const updated = { ...places[place.id], ...place } as Place;
	const resp = await fetch(`/api/v1/places/${place.id}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		},
		// The API's update_place replaces all place columns and expects the full
		// (flattened) ApiPlace payload, including image_ids and reservations.
		body: JSON.stringify({
			id: updated.id,
			name: updated.name,
			description: updated.description ?? null,
			in_place: updated.in_place ?? null,
			in_department: updated.in_department ?? null,
			main_img: updated.main_img ?? null,
			image_ids: updated.image_ids ?? [],
			reservations: updated.reservations ?? []
		})
	});
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to update place');
	}
	places[place.id] = { ...places[place.id], ...place };
	return places[place.id];
}

export async function fetchPlace(id: number): Promise<Place> {
	const resp = await fetch(`/api/v1/places/${id}`);
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to fetch place');
	}
	const placeResponse: PlaceResponse = await resp.json();
	const place = addImageUrls(placeResponse);
	places[place.id] = place;
	return place;
}

export async function deletePlace(id: number): Promise<void> {
	const resp = await fetch(`/api/v1/places/${id}`, {
		method: 'DELETE'
	});
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to delete place');
	}
	delete places[id];
}

export async function deletePlaceImage(placeId: number, imageId: string): Promise<void> {
	const resp = await fetch(`/api/v1/places/${placeId}/images/${imageId}`, {
		method: 'DELETE'
	});
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to delete place image');
	}
}

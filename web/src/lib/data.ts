export interface Thing {
	id: number;
	count: number;
	in_place?: number;
	department?: number;
	name: string;
	description: string;
	main_img?: string;
	label_ids: number[];
	image_ids: string[];
	reservations: {
		id: number;
		count: number;
		reserved_by: string;
	}[];
}

export interface Place {
	id: number;
	name: string;
	description: string;
	in_place?: number;
	in_department?: number;
	main_img?: string;
	image_ids: string[];
	reservations: {
		id: number;
		reserved_by: string;
	}[];
}

export interface Label {
	id: number;
	name: string;
	description?: string;
	color?: string;
}

export interface Department {
	id: number;
	name: string;
	main_img?: string;
	image_ids: string[];
}

export class FetchError extends Error {
	code: number;

	constructor(code: number, message: string) {
		super(message);
		this.code = code;
	}
}

const fetchPlaces = async () => {
	const resp = await fetch(`/api/v1/places`);
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to fetch places');
	}

	const places: Place[] = await resp.json();

	return places;
};
export const places = fetchPlaces().then((places) =>
	places.reduce(
		(places, place) => {
			places[place.id] = place;
			return places;
		},
		{} as Record<number, Place>
	)
);

const fetchDepartments = async () => {
	const resp = await fetch(`/api/v1/departments`);
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to fetch departments');
	}

	const departments: Department[] = await resp.json();

	return departments;
};
export const departments = fetchDepartments().then((departments) =>
	departments.reduce(
		(departments, dep) => {
			departments[dep.id] = dep;
			return departments;
		},
		{} as Record<number, Department>
	)
);

const fetchLabels = async () => {
	const resp = await fetch(`/api/v1/labels`);
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to fetch labels');
	}

	const labels: Label[] = await resp.json();

	return labels;
};
export const labels = fetchLabels().then((labels) =>
	labels.reduce(
		(result, label) => {
			result[label.id] = label;
			return result;
		},
		{} as Record<number, Label>
	)
);

export async function newLabel(name: string): Promise<Label> {
	const response = await fetch('/api/v1/labels', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ name })
	});

	if (!response.ok) {
		throw new FetchError(response.status, 'Failed to create label');
	}

	const label: Label = await response.json();
	const l = await labels;

	l[label.id] = label;

	return label;
}

const fetchThings = async () => {
	const resp = await fetch(`/api/v1/things`);
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to fetch things');
	}

	const things: Thing[] = await resp.json();

	return things;
};
export const things = fetchThings();

export const fetchThing = async (id: number) => {
	const resp = await fetch(`/api/v1/things/${id}`);
	if (!resp.ok) {
		throw new FetchError(resp.status, resp.statusText || 'Failed to fetch thing');
	}

	const things: Thing = await resp.json();

	return things;
};

export async function updateThing(thing: Thing): Promise<void> {
	const response = await fetch(`/api/v1/things/${thing.id}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(thing)
	});

	if (!response.ok) {
		throw new FetchError(response.status, 'Failed to update thing');
	}
}

export async function newThing(thing: {
	name: string;
	description: string;
	count?: number;
	in_department?: number;
	in_place?: number;
}): Promise<Thing> {
	const response = await fetch('/api/v1/things', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(thing)
	});

	if (!response.ok) {
		throw new FetchError(response.status, 'Failed to create thing');
	}

	return response.json();
}

export async function newPlace(place: {
	name: string;
	description: string;
	in_department?: number;
	in_place?: number;
}): Promise<Place> {
	const response = await fetch('/api/v1/places', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(place)
	});

	if (!response.ok) {
		throw new FetchError(response.status, 'Failed to create place');
	}

	return response.json();
}

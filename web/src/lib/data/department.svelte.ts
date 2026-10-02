import { FetchError } from '$lib/data.svelte';

export interface Department {
	id: number;
	name: string;
	main_img?: string | null;
	main_url?: string;
	image_ids: string[];
	image_urls: Record<string, string>;
	order: number;
}

type DepartmentResponse = Omit<Department, 'main_url' | 'image_urls'> & { order?: number };

async function fetchDepartmentDetails(id: number) {
	const resp = await fetch(`/api/v1/departments/${id}`);
	if (!resp.ok) {
		throw new FetchError(resp.status, `Failed to fetch department ${id}`);
	}

	return (await resp.json()) as DepartmentResponse;
}

async function addImageUrls(department: DepartmentResponse): Promise<Department> {
	const image_urls: Record<string, string> = {};
	await Promise.all(
		department.image_ids.map(async (id) => {
			const resp = await fetch(`/api/v1/departments/${department.id}/images/${id}`);
			if (resp.ok) {
				image_urls[id] = URL.createObjectURL(await resp.blob());
			}
		})
	);

	return {
		...department,
		image_urls,
		main_url: department.main_img ? image_urls[department.main_img] : undefined
	} as Department;
}

export async function fetchDepartments() {
	const resp = await fetch(`/api/v1/departments`);
	if (!resp.ok) {
		throw new FetchError(resp.status, 'Failed to fetch departments');
	}

	const departments: DepartmentResponse[] = await resp.json();
	const depOrder = departments
		.sort((a, b) => a.name.localeCompare(b.name))
		.map((dep, index) => {
			dep.order = index;
			return dep;
		});
	return Promise.all(depOrder.map(addImageUrls));
}

export const departments = $state<Record<number, Department>>(
	await fetchDepartments().then((departments) => {
		return departments.reduce(
			(departments, dep) => {
				departments[dep.id] = dep;
				return departments;
			},
			{} as Record<number, Department>
		);
	})
);

export async function updateDepartment(
	department: Department & { newImage?: FileList; imagesToDelete?: string[] }
) {
	const newImage = department.newImage?.[0];

	delete department.newImage;
	const imagesToDelete = department.imagesToDelete ?? [];
	delete department.imagesToDelete;

	const updateResp = await fetch(`/api/v1/departments/${department.id}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			id: department.id,
			name: department.name,
			main_img: department.main_img ?? null,
			image_ids: department.image_ids,
			order: department.order
		})
	});
	if (!updateResp.ok) {
		throw new FetchError(updateResp.status, 'Failed to update department');
	}

	if (newImage) {
		await uploadDepartmentImage(department.id, newImage);
	}
	await Promise.all(
		imagesToDelete.map(async (imageId) => {
			await deleteDepartmentImage(department.id, imageId);
		})
	);

	const updatedDepartment = await addImageUrls(await fetchDepartmentDetails(department.id));
	departments[department.id] = updatedDepartment;
}

async function uploadDepartmentImage(departmentId: number, image: File) {
	return new Promise<void>(async (resolve, reject) => {
		const uploadResp = await fetch(`/api/v1/departments/${departmentId}/images`, {
			method: 'POST',
			headers: { 'Content-Type': image.type || 'application/octet-stream' },
			body: image
		});
		if (!uploadResp.ok) {
			reject(new FetchError(uploadResp.status, 'Failed to upload department image'));
		}
		resolve();
	});
}

async function deleteDepartmentImage(departmentId: number, imageId: string) {
	const deleteResp = await fetch(`/api/v1/departments/${departmentId}/images/${imageId}`, {
		method: 'DELETE'
	});
	if (!deleteResp.ok) {
		throw new FetchError(deleteResp.status, 'Failed to delete department image');
	}
}

export async function deleteDepartment(departmentId: number) {
	const deleteResp = await fetch(`/api/v1/departments/${departmentId}`, {
		method: 'DELETE'
	});
	if (!deleteResp.ok) {
		throw new FetchError(deleteResp.status, 'Failed to delete department');
	}
	delete departments[departmentId];
}

export async function createDepartment(name: string, image?: File) {
	const createResp = await fetch(`/api/v1/departments`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			name
		})
	});
	if (!createResp.ok) {
		throw new FetchError(createResp.status, 'Failed to create department');
	}
	const newID = (await createResp.json()).id;
	if (image) await uploadDepartmentImage(newID, image);

	const newDepartment = await fetchDepartmentDetails(newID);
	let parsedNewDepartment = await addImageUrls(newDepartment);

	departments[newID] = parsedNewDepartment;
	Object.values(departments)
		.sort((a, b) => a.name.localeCompare(b.name))
		.map((dep, index) => {
			dep.order = index;
			return dep;
		});
}

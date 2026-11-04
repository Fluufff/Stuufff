export const MAX_IMAGE_SIZE = 2 * 1024 * 1024; // 2MB

const QUALITY_STEPS = [0.9, 0.8, 0.7, 0.5];

const SCALE_STEP = 0.8;

function canvasToBlob(
	canvas: HTMLCanvasElement,
	type: string,
	quality: number
): Promise<Blob | null> {
	return new Promise((resolve) => canvas.toBlob(resolve, type, quality));
}

export async function resizeImageIfNeeded(file: File, maxBytes = MAX_IMAGE_SIZE): Promise<File> {
	if (file.size <= maxBytes) return file;
	if (file.type === 'image/gif' || file.type === 'image/svg+xml') return file;

	let bitmap: ImageBitmap;
	try {
		bitmap = await createImageBitmap(file, { imageOrientation: 'from-image' });
	} catch {
		return file;
	}

	const canvas = document.createElement('canvas');
	const canvasContext = canvas.getContext('2d');
	if (!canvasContext) {
		bitmap.close();
		return file;
	}

	let width = bitmap.width;
	let height = bitmap.height;
	let blob: Blob | null = null;

	try {
		for (const quality of QUALITY_STEPS) {
			canvas.width = width;
			canvas.height = height;
			canvasContext.clearRect(0, 0, width, height);
			canvasContext.drawImage(bitmap, 0, 0, width, height);
			blob = await canvasToBlob(canvas, 'image/webp', quality);
			if (!blob) return file;
			if (blob.size <= maxBytes) break;
			width = Math.max(1, Math.round(width * SCALE_STEP));
			height = Math.max(1, Math.round(height * SCALE_STEP));
		}
	} finally {
		bitmap.close();
	}

	if (!blob || blob.size >= file.size) return file;

	const baseName = file.name.replace(/\.[^.]+$/, '');
	return new File([blob], `${baseName}.webp`, {
		type: 'image/webp',
		lastModified: file.lastModified
	});
}

export async function processFiles(fileList: FileList | null): Promise<FileList> {
	const dt = new DataTransfer();
	if (fileList?.length) {
		const processed = await Promise.all(
			Array.from(fileList).map((file) => resizeImageIfNeeded(file))
		);
		for (const file of processed) dt.items.add(file);
	}
	return dt.files;
}

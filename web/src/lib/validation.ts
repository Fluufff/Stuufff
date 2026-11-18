export function isBlank(value: unknown): boolean {
	if (value == null) return true;
	if (Array.isArray(value)) return value.length === 0;
	if (typeof value === 'string') return value.trim() === '';
	return false;
}

export function isValidEmail(value: string): boolean {
	return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value);
}

export function isValidUrl(value: string): boolean {
	try {
		const url = new URL(value);
		return url.protocol === 'http:' || url.protocol === 'https:';
	} catch {
		return false;
	}
}

export function isValidHexColor(value: string): boolean {
	return /^#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})$/.test(value);
}

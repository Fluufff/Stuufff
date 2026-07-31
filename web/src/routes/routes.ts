import { resolve } from '$app/paths';

export const routes = [
	{
		// URL path in web UI
		path: resolve('/inventory'),
		// Icon class, see src/lib/icons.css
		icon: 'icon-[lucide--container] text-[32px]',
		// Menu name in web UI
		name: 'Inventory'
	}
];

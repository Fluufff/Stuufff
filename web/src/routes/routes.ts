import { resolve } from '$app/paths';

export const routes = [
	{
		// URL path in web UI
		path: resolve('/inventory'),
		// Icon class, see src/lib/icons.css
		icon: 'icon-[mdi--archive-outline] text-[32px]',
		// Menu name in web UI
		name: 'Inventory'
	},
	{
		// URL path in web UI
		path: resolve('/reservations'),
		// Icon class, see src/lib/icons.css
		icon: 'icon-[mdi--truck-fast-outline] text-[32px]',
		// Menu name in web UI
		name: 'Reservations'
	},
	{
		// URL path in web UI
		path: resolve('/audit'),
		// Icon class, see src/lib/icons.css
		icon: 'icon-[mdi--archive-clock-outline] text-[32px]',
		// Menu name in web UI
		name: 'Audit logs'
	}
];

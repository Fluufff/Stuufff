export const icon = {
	check: 'icon-[material-symbols--check]',
	close: 'icon-[material-symbols--close]',
	edit: 'icon-[material-symbols--edit]'
} as const;

export type icon = keyof typeof icon;

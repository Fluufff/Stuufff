export const icon = {
	check: 'icon-[material-symbols--check]',
	close: 'icon-[material-symbols--close]',
	edit: 'icon-[material-symbols--edit]',
	delete: 'icon-[material-symbols--delete]',
	plus: 'icon-[material-symbols--add]'
} as const;

export type icon = keyof typeof icon;

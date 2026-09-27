let tabs = $state([
	{ label: 'Departments', active: false, id: 1 },
	{ label: 'Locations', active: false, id: 2 },
	{ label: 'Things', active: true, id: 3 }
]);
let activeTab = $derived(tabs.find((tab) => tab.active))!!;

export function getTabs() {
	return tabs;
}

export function setActiveTab(id: number) {
	tabs = tabs.map((tab) => ({ ...tab, active: tab.id === id }));
}

export function getActiveTab() {
	return activeTab;
}

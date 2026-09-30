export function teleport(node: HTMLElement, name: string = 'teleport') {
	let teleportContainer = document.getElementById(name);
	if (!teleportContainer) {
		teleportContainer = document.createElement('div');
		teleportContainer.id = name;
		document.body.appendChild(teleportContainer);
	}
	teleportContainer.appendChild(node);

	return {
		destroy() {
			node.remove();
		}
	};
}

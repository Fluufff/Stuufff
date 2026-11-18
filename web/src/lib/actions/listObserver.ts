type Callback = (element: HTMLElement, observer: IntersectionObserver) => void;
type Params = { callback: Callback; rootMargin?: number; rootElement?: string; once?: boolean };

export function observeListElement(element: HTMLElement, params: Params) {
	const margin = `0px 0px ${params.rootMargin ?? 300}px 0px`;
	const root =
		(params.rootElement ? document.getElementById(params.rootElement) : null) ?? document;

	const observer = new IntersectionObserver(
		(entries) => {
			for (const entry of entries) {
				if (!entry.isIntersecting) continue;
				params.callback(element, observer);
				if (params.once) observer.unobserve(element);
			}
		},
		{ root, rootMargin: margin, threshold: 0 }
	);

	observer.observe(element);

	return {
		destroy() {
			observer.disconnect();
		}
	};
}

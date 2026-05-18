import { signal, effect, tick } from "https://esm.sh/@maverick-js/signals@6.0.0";

const signals = new Map();

function scanSignals(root = document.body) {
	const walker = document.createTreeWalker(root, NodeFilter.SHOW_COMMENT);

	for (let node = walker.nextNode(); node; node = walker.nextNode()) {
		const match = node.data.match(/^\s*signal:\s*(\{.*\})\s*$/);
		if (!match) continue;

		const { id, value } = JSON.parse(match[1]);
		if (signals.has(id)) {
			signals.get(id).set(value);
		} else {
			signals.set(id, signal(value));
		}
	}
}

scanSignals();

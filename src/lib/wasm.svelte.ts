import init, { draw } from 'rust';

let initialized = $state(false);

export function isInitialized() {
	return initialized;
}

export async function initWasm() {
	await init();
	initialized = true;
}

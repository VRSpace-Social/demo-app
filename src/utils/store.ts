// store.ts
import { getContext, setContext } from 'svelte';
import { type Writable, writable } from 'svelte/store';

export function createStore(value: string) {
	const mystore = writable(value);
	setContext('contextStore', mystore);
}

export function readStore() {
	return getContext<Writable<string>>('contextStore');
}

export function updateStore(value: string) {
	readStore().set(value);
}
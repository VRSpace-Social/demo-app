// src/lib/toastStore.ts
import { writable } from 'svelte/store';

type Toast = {
  message: string;
  id: number;
};

export const toasts = writable<Toast[]>([]);

let id = 0;

export function addToast(message: string, duration = 3000) {
  id += 1;
  toasts.update((all) => [...all, { message, id }]);

  setTimeout(() => {
    toasts.update((all) => all.filter((toast) => toast.id !== id));
  }, duration);
}

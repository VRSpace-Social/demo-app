// src/utils/toast.ts
import { addToast } from '$lib/toastStore';

export function showToast(message: string) {
  addToast(message);
}

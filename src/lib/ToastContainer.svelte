<!-- src/lib/ToastContainer.svelte -->
<script lang="ts">
  import { toasts } from './toastStore';
  import Toast from './Toast.svelte';
  import { onDestroy } from 'svelte';

  let toastsList: Array<any> = [];

  const unsubscribe = toasts.subscribe(value => {
    toastsList = value;
  });

  onDestroy(() => {
    unsubscribe();
  });
</script>

<div>
  {#each toastsList as toast (toast.id)}
    <Toast message={toast.message} />
  {/each}
</div>

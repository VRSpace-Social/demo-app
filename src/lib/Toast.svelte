<!-- src/lib/Toast.svelte -->
<script lang="ts">
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store';
  
    export let message: string = '';
    export let duration: number = 3000;
  
    const visible = writable(false);
  
    onMount(() => {
      visible.set(true);
      setTimeout(() => {
        visible.set(false);
      }, duration);
    });
  </script>
  
  <style>
    .toast {
      position: fixed;
      bottom: 20px;
      left: 50%;
      transform: translateX(-50%);
      background-color: #333;
      color: #fff;
      padding: 10px 20px;
      border-radius: 5px;
      box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
      opacity: 0;
      transition: opacity 0.5s ease-in-out;
    }
  
    .toast.visible {
      opacity: 1;
    }
  </style>
  
  <div class="toast" class:visible={$visible}>
    {message}
  </div>
  
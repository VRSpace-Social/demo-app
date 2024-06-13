<!-- src/lib/ReadOnlyDiv.svelte -->
<script lang="ts">
    export let texts: Array<String> = [];
  </script>
  
  <style>
    .readonly-div {
      white-space: nowrap; /* Evita che il testo vada a capo */
      pointer-events: none; /* Rende la div non interattiva */
      border: 1px solid black; /* Aggiungi un bordo per vedere la div */
      padding: 10px; /* Aggiungi un po' di padding per migliorare l'aspetto */
      overflow: auto; /* Aggiungi uno scroll se il contenuto supera la dimensione della div */
    }
  </style>
  
  <div class="readonly-div">
    {#each texts as text}
      <div>{text}</div>
    {/each}
  </div>
  
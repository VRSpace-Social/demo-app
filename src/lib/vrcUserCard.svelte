<script lang="ts">
    import Icon from '@iconify/svelte';
    import "../app.css";

    export let username;
    export let profilePicture;
    export let worldPicture;
    export let instanceType;
    export let playerCounter;
    export let maxPlayerCount;
    export let canJoin: boolean;
    export let instanceId: string;
    export let worldName;
    export let vrcInstanceUrl: string;

    export async function getInstance(instanceId: string) {
        const response = await fetch(`http://localhost:3000/api/getInstance/${instanceId}`);
        return await response.json();
    }
</script>

<main class="flex flex-wrap justify-center">
    <div
     style="background: url({worldPicture}); background-repeat: no-repeat; background-size: cover;"
     class="flex items-end rounded-2xl bg-center bg-cover pt-36 pr-5 pb-4 pl-4 foreground-overlay m-4">
     
     {#if canJoin}
     <div class="flex justify-end rounded-xl bg-green-500 p-4 m-2">
        <div class="flex h-8 w-8 shrink-0 items-center justify-center">
            <a href="{vrcInstanceUrl}" target="_blank"><Icon icon="mdi:play" class="scale-[2.0] hover:scale-[3.0]" /></a>
        </div>
      </div>
      {:else}
      <div class="flex justify-end rounded-xl p-4 m-2" style="background-image: url({profilePicture}); background-repeat: no-repeat; background-size: cover;">
        <div class="flex h-8 w-8 shrink-0 items-center justify-center"></div>
      </div>
      {/if}
  
  <div class="text-white text-xs font-medium">
    <p class="text-xs text-white font-medium">
      <span>{username}</span>
      <span class="text-neutral-100">is playing:</span>
    </p>
    <h2 class="text-2xl font-bold">{worldName}</h2>
    <div class="flex flex-col">
      <p class="flex items-center"><Icon icon="mdi:account-group-outline" class="m-1" />{playerCounter}/{maxPlayerCount}</p>
      <p class="flex items-center">{instanceType}</p>
    </div>
  </div>
</div>
</main>

<style>
  .foreground-overlay {
    background: linear-gradient(to bottom, rgba(0, 0, 0, 0.5), rgba(0, 0, 0, 0));
  }
</style>

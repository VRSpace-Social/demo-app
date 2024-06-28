<script lang="ts">
  import { onMount, beforeUpdate } from "svelte";
  import ToastContainer from '$lib/ToastContainer.svelte';
  import { showToast } from '$utils/toast';
  import { createStore, updateStore } from '$utils/store';

  import "../app.css";

  import { AppBar, Button, Menu, MenuItem } from 'svelte-ux';
  import {mdiRefresh, mdiHome, mdiInformation, mdiAccountGroup, mdiWeb, mdiMenu} from '@mdi/js';


  let menuOpen = false;

  // Setup a global state variable to store if the VRSpace server is running, so that other components can access it.
  let isVrSpaceServerRunning = false;
  let storeValue: string = String(isVrSpaceServerRunning);

   beforeUpdate(() => {
      console.log("Insert any code you want to run before the page updates here.");
      storeValue = String(isVrSpaceServerRunning);
      updateStore(storeValue);
  });

  onMount(async () => {
      console.log("Insert any code you want to run when the page loads here.");
      const hiRes = await fetch("http://localhost:3000/hi").catch(e => {
          if(e instanceof TypeError && e.message === "Failed to fetch") 
              return new Response("not running");
          else 
              throw e;            
      });
      if(await hiRes.text() === "hello!") {
          console.log("VRSpace server is running!");
          isVrSpaceServerRunning = true;
      } else {
          console.log("VRSpace server is not running!");
          isVrSpaceServerRunning = false;
          showToast("VRSpace server appears to be not running!");
          const vrcWsElement = document.getElementById("vrc-ws");
          if (vrcWsElement !== null) {
              vrcWsElement.style.display = "none";
          }
      }
      storeValue = String(isVrSpaceServerRunning);
  });

  createStore(storeValue);
</script>


<AppBar title="VRSpace">
  <div slot="actions">
    <!-- Bottone Home -->
    <Button icon={mdiHome} href="/" label="Home" class="p-2 hover:bg-surface-100/10" />
    
    <!-- Bottone About -->
    <Button icon={mdiInformation} href="/about" label="About" class="p-2 hover:bg-surface-100/10" />
    
    <!-- Bottone VRC Friends -->
    <Button icon={mdiAccountGroup} href="/friends" label="VRC Friends" class="p-2 hover:bg-surface-100/10" />
    
    <!-- Bottone VRChat WebSocket -->
    <Button icon={mdiWeb} href="/websocket" id="vrc-ws" label="VRChat WebSocket" class="p-2 hover:bg-surface-100/10">WebSocket</Button>
  </div>
</AppBar>

<!--
<nav class="navbar">
<a href="/">Home</a>
<a href="/about">About</a>
<a href="/friends">VRC Friends</a>
<a id="vrc-ws" href="/websocket">VRChat WebSocket</a>
</nav>
-->

<main>
<slot></slot>
<ToastContainer />
</main>
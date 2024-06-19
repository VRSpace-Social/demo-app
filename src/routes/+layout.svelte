<script lang="ts">
    import { onMount, afterUpdate, beforeUpdate } from "svelte";
    import ToastContainer from '$lib/ToastContainer.svelte';
    import { showToast } from '$utils/toast';
    import { createStore, updateStore } from '$utils/store';
    import { invoke } from '@tauri-apps/api/tauri';

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

<nav class="navbar">
  <a href="/">Home</a>
  <a href="/about">About</a>
  <a href="/friends">VRC Friends</a>
  <a id="vrc-ws" href="/websocket">VRChat WebSocket</a>
</nav>

<main>
  <slot></slot>
  <ToastContainer />
</main>

<style>
  .navbar {
    display: flex;
    justify-content: center;
    gap: 20px;
    padding: 1em;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  .navbar a {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    background-color: #0f0f0f;
    color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  .navbar a:hover {
    color: #396cd8;
  }

  .navbar a:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }
</style>

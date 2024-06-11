<script lang="ts">
    import { onMount } from "svelte";
    import { navigating } from '$app/stores';

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
        } else {
            console.log("VRSpace server is not running!");
            const vrcWsElement = document.getElementById("vrc-ws");
            if (vrcWsElement !== null) {
                vrcWsElement.style.display = "none";
            }
        }
    });

    $: if($navigating) {
        console.log($navigating);
        if($navigating.from?.route.id === "/websocket") {
            
        }
    };
</script>

<nav class="navbar">
	<a href="/">Home</a>
	<a href="/about">About</a>
    <a id="vrc-ws" href="/websocket">VRChat WebSocket</a>
</nav>

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

<slot> 

</slot>


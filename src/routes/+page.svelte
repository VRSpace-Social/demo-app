<script lang="ts">
  import {fetchData} from '$utils/fetch';
  import { invoke } from "@tauri-apps/api/tauri";
  import { Card, Button } from 'svelte-ux';

  let url = "";
  let username = "";
  let password = "";
  let twofactorcode = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await sendNotif();
    const response = await fetchData(url);
    console.log(response);
  }


  async function vrc_test() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke("vrc_test", { username: username, password: password, twofactorcode: twofactorcode })
      .then(async () => {
        console.log("Sent start event to Rust VRC client");
      })
      .catch((error: any) => {
        console.error("Failed to start VRC:", error);
      });
  }



  import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
  async function sendNotif() {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
      sendNotification({ 
        sound: 'Mail',
        icon: 'icons/128x128@2x.png',
        title: 'TAURI', 
        body: 'Tauri is awesome!'
      });
    }
  }
</script>

<div class="container">
  <title>VRSpace</title>
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>

  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Test URL to fetch..." bind:value={url} />
    <button type="submit">Greet</button>
  </form>


  <form class="row" on:submit|preventDefault={vrc_test}>
    <input id="username-input" placeholder="usernsme" bind:value={username} />
    <input id="password-input" type="password" placeholder="password" bind:value={password} />
    <input id="twofactorcode-input" placeholder="twofactorcode" bind:value={twofactorcode} />
    <button type="submit">Test VRC</button>
  </form>

  <div class="grid grid-cols-2 gap-3">
    <Card title="Title" subheading="with actions">
      <div slot="actions">
        <Button>Action 1</Button>
        <Button>Action 2</Button>
      </div>
    </Card>
    <Card title="Title" subheading="with content" loading>
      <div slot="contents" class="bg-danger/10">Contents</div>
    </Card>
  </div>
</div>



<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
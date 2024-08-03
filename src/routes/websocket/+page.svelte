<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { fetchData } from "$utils/fetch";
  import { onMount, onDestroy } from "svelte";
  import { showToast } from "$utils/toast";
  import ReadOnlyDiv from "$lib/ReadOnlyDiv.svelte";
  import { goto } from "$app/navigation";

  let inputValue: string = "";
  let texts: Array<string> = [];
  let wsTimer: number;
  let timerStart: Date;
  let timerEnd: Date;
  let bkAuthCookie: string;
  let isWSRunning: boolean;

  // Store the unlisten functions to remove listeners later
  let unlistenFns: (() => void)[] = [];

  async function getInstanceInfo(worldId: string, instanceId: string) {
    const result = await fetch(
      "http://localhost:3000/api/getInstanceInfo?worldID=" +
        worldId +
        "&instanceID=" +
        instanceId
    );
    const jsonResult = await result.json();
    return jsonResult;
  }

  async function getUserInfo(userID: string) {
    const result = await fetch(
      "http://localhost:3000/api/getUserInfo?userID=" + userID
    );
    return result.json();
  }

  async function startWS(bkAuthCookie: string) {
    // Remove previous listeners
    for (const unlisten of unlistenFns) {
      await unlisten();
    }
    unlistenFns = [];

    invoke("start_ws", { authcookie: bkAuthCookie })
      .then(async () => {
        console.log("Sent start event to Rust WS client");
      })
      .catch((error: any) => {
        console.error("Failed to start WebSocket:", error);
      });

    const unlistenWsConnected = await listen("ws_connected", async (event) => {
      if (event.payload === "ws_connected") {
        console.log("WebSocket connected!");
        timerStart = new Date();
        texts = [...texts, "Connected to WebSocket server"];
        isWSRunning = true;
        await handleWSMsg();
      }
    });
    unlistenFns.push(unlistenWsConnected);

    const unlistenWsErr = await listen("ws_err", async (event: any) => {
      if (event.payload.err) {
        console.error("WebSocket VRChat ERROR: " + event.payload.err);
        texts = [...texts, "WebSocket VRChat ERROR: " + event.payload.err];
      } else {
        console.error("WebSocket LOGIC error: " + event.payload);
        texts = [...texts, "WebSocket LOGIC error: " + event.payload];
      }
      await invoke("stop_ws")
        .then((result) => {
          if (result) {
            console.log("WebSocket stopped");
            console.log("Disconnected from WebSocket server");
            texts = [...texts, "Disconnected from WebSocket server"];
            isWSRunning = false;
          } else {
            console.log("WebSocket was not running");
          }
        })
        .catch((error) => {
          console.error("Failed to stop WebSocket:", error);
        });
    });
    unlistenFns.push(unlistenWsErr);
  }

  async function handleWSMsg() {
    const unlistenWsMsg = await listen("ws_msg", async (event) => {
      let wsMsg: any = event.payload;
      console.log(wsMsg);

      if (wsMsg.err) {
        texts = [...texts, wsMsg.err];
      }

      let msgType = wsMsg.type;
      let msgContent = JSON.parse(wsMsg.content);

      switch (msgType) {
        case "friend-active":
          console.log(
            msgContent.user.displayName +
              " is now active (online on VRC Website or API)"
          );
          showToast(
            msgContent.user.displayName +
              " is now active (online on VRC Website or API)"
          );
          texts = [
            ...texts,
            msgContent.user.displayName +
              " is now active (online on VRC Website or API)",
          ];
          break;

        case "friend-offline":
          let friendData = await getUserInfo(msgContent.userId).catch((e) => {
            let errorResponse = JSON.parse(e.response?.data);
            console.log(errorResponse);
            console.error(
              "[!!] ERROR on" +
                wsMsg +
                "\nError while trying to get friend data: \n" +
                e
            );
            return undefined;
          });
          if (friendData === undefined) {
            console.warn("Friend is not found");
            break;
          }
          let friendUsername = friendData.displayName;
          console.log(friendUsername + " is pending offline");
          showToast(friendUsername + " is pending offline");
          texts = [...texts, friendUsername + " is pending offline"];
          break;

        case "friend-delete":
          let friendDataDelete = await getUserInfo(msgContent.userId).catch(
            (e) => {
              let errorResponse = JSON.parse(e.response?.data);
              console.log(errorResponse);
              console.error(
                "[!!] ERROR on" +
                  wsMsg +
                  "\nError while trying to get friend data: \n" +
                  e
              );
              return undefined;
            }
          );
          if (friendDataDelete === undefined) {
            console.warn("Friend is not found");
            break;
          }
          let friendUsernameDelete = friendDataDelete.displayName;
          console.log(friendUsernameDelete + " is no longer your friend");
          showToast(friendUsernameDelete + " is no longer your friend");
          texts = [
            ...texts,
            friendUsernameDelete + " is no longer your friend",
          ];
          break;

        case "friend-online":
          let friendUsernameOnline = msgContent.user.displayName;
          console.log(friendUsernameOnline + " is online on VRChat Client");
          showToast(friendUsernameOnline + " is online on VRChat Client");
          texts = [
            ...texts,
            friendUsernameOnline + " is online on VRChat Client",
          ];
          break;

        default:
          try {
            console.warn(
              "PLEASE CHECK, NEEDS TO BE IMPLEMENTED, TYPE: " + msgType
            );
            console.log(msgContent);
          } catch (e) {
            console.error("Error while trying to parse message: " + e);
            console.log(wsMsg);
          }
      }
    });
    unlistenFns.push(unlistenWsMsg);
  }

  function doRunWS(authCookie: string) {
    if (!isWSRunning) {
      startWS(authCookie);
    }
  }

  async function stopWS() {
    await invoke("stop_ws")
        .then((result) => {
          if (result) {
            console.log("WebSocket stopped");
            console.log("Disconnected from WebSocket server");
            texts = [...texts, "Disconnected from WebSocket server"];
            isWSRunning = false;
          } else {
            console.log("WebSocket was not running");
          }
        })
        .catch((error) => {
          console.error("Failed to stop WebSocket:", error);
        });
  }

  onMount(async () => {
    console.log("Welcome to VRChat WebSocket using VRSpace API!");
    const authCookie: string | undefined = await fetchData(
      "http://localhost:3000/api/getAuthCookie",
      true
    ).catch((e) => {
      console.error("Error while trying to get auth cookie: " + e);
      return undefined;
    });
    if (authCookie) {
      if (authCookie === "Not Found :(") {
        console.error("Auth cookie is not found");
        showToast("Auth cookie is not found");
        texts = [...texts, "Auth cookie is not found"];
        invoke("stop_ws");
        return;
      }
      bkAuthCookie = authCookie;
      await startWS(bkAuthCookie)
        .then(async () => {
          console.log("Starting VRC WebSocket...");
        })
        .catch((e) => {
          console.error("Error while trying to start WebSocket: " + e);
        });
    } else {
      console.error("Auth cookie is not found");
      showToast("Auth cookie is not found");
      texts = [...texts, "Auth cookie is not found"];
    }
  });

  onDestroy(async () => {
    for (const unlisten of unlistenFns) {
      await unlisten();
    }
    await invoke("stop_ws")
      .then((result) => {
        if (result) {
          console.log("WebSocket stopped");
          timerEnd = new Date();
          console.log("Disconnected from WebSocket server");
          showToast("Disconnected from WebSocket server");
          texts = [...texts, "Disconnected from WebSocket server"];
          wsTimer = Math.floor(
            (timerEnd.getTime() - timerStart.getTime()) / 1000
          );
          texts = [
            ...texts,
            "WebSocket was running for " + wsTimer + " seconds",
          ];
          isWSRunning = false;
        } else {
          console.log("WebSocket was not running");
        }
      })
      .catch((error) => {
        console.error("Failed to stop WebSocket:", error);
      });
  });

  function handleSubmit() {
    if (inputValue) {
      console.log(inputValue);
      goto(`/about/${inputValue}`);
    }
  }

  function reRunWS() {
    if (!isWSRunning) {
      doRunWS(bkAuthCookie);
    }
  }
</script>

<h1>VRChat WebSocket using VRSpace API</h1>

<button on:click={stopWS}>Stop WS</button>

<button on:click={reRunWS}>Restart WS</button>

<form action="" on:submit|preventDefault={handleSubmit}>
  <input type="text" bind:value={inputValue} />
</form>

<ReadOnlyDiv {texts} />

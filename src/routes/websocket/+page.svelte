<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { fetchData } from "$utils/fetch";

  /*
  await listen("ws_connected", (event) => {
      if(event.payload === "ws_connected") console.log("WebSocket connected!");
    });

    await listen("ws_msg", (event) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      console.log(event.payload);
    });

    await listen("ws_disconnected", (event) => {
      if(event.payload === "ws_disconnected") console.log("WebSocket disconnected!");
    });
  */

  let inputValue: string = "";

  let texts: Array<string> = [];

  let wsTimer: number;
  let timerStart: Date;
  let timerEnd: Date;

  import { onMount, onDestroy } from "svelte";
  import { showToast } from "$utils/toast";
  import ReadOnlyDiv from "$lib/ReadOnlyDiv.svelte";
  import { goto } from "$app/navigation";

  let bkAuthCookie: string;
  let isWSRunning: boolean;
  let webSocket: WebSocket; // Declare webSocket as a global variable

  async function getInstanceInfo(worldId: string, instanceId: string) {
    debugger;
    //TODO: Implement this function
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
    invoke("start_ws", { authcookie: bkAuthCookie })
      .then(async () => {
        console.log("Sent start event to Rust WS client");
      })
      .catch((error: any) => {
        console.error("Failed to start WebSocket:", error);
      });

    await listen("ws_connected", async (event) => {
      if (event.payload === "ws_connected") {
        console.log("WebSocket connected!");
        timerStart = new Date();
        texts = [...texts, "Connected to WebSocket server"];
        isWSRunning = true;
        await handleWSMsg();
      }
    });
  }

  async function handleWSMsg() {
    await listen("ws_msg", async (event) => {
      // Handling of WebSocket messages
      let wsMsg: any = event.payload;
      console.log(wsMsg);

      // WS Message error
      if (wsMsg.err) {
        texts = [...texts, wsMsg.err];
      }
      let msgType = wsMsg.type;
      let msgContent = JSON.parse(wsMsg.content);
      // Here we can handle different types of messages via a switch statement
      switch (msgType) {
        case "friend-active": {
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
        }

        case "friend-offline": {
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
        }

        case "friend-delete": {
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
          console.log(friendUsername + " is no longer your friend");
          showToast(friendUsername + " is no longer your friend");
          texts = [...texts, friendUsername + " is no longer your friend"];
          break;
        }

        case "friend-online": {
          let friendUsername = msgContent.user.displayName;
          console.log(friendUsername + " is online on VRChat Client");
          showToast(friendUsername + " is online on VRChat Client");
          texts = [...texts, friendUsername + " is online on VRChat Client"];
          break;
        }

        default: {
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
      }
    });

    /*
    webSocket.onmessage = async function (event) {
      switch (JSON.parse(event.data.toString()).type) 
        case "friend-location": {
          let jsonData = JSON.parse(JSON.parse(event.data.toString()).content);
          let location = jsonData.location;
          let travelingToLocation = jsonData.travelingToLocation;
          console.log(location);
          console.log(travelingToLocation);

          if (
            location === "traveling" &&
            travelingToLocation !== "" &&
            jsonData.worldId.substring(0, 5) === "wrld_"
          ) {
            // User IS TRAVELING to instance
            debugger;
            let worldId = jsonData.worldId;
            let instanceId = jsonData.travelingToLocation.split(":")[1];
            const resp = await getInstanceInfo(worldId, instanceId);
            let instanceData = await getInstanceInfo(worldId, instanceId).catch(
              (e) => {
                console.log(e.response);
                console.error("Error while trying to get instance data: " + e);
                return undefined;
              }
            );
            console.log(
              "User (" +
                jsonData.user.displayName +
                ") is traveling to world: [" +
                instanceData?.world.name +
                "]"
            );
            showToast(
              "User (" +
                jsonData.user.displayName +
                ") is traveling to world: [" +
                instanceData?.world.name +
                "]"
            );
            texts = [
              ...texts,
              "User (" +
                jsonData.user.displayName +
                ") is traveling to world: [" +
                instanceData?.world.name +
                "]",
            ];
          } else if (
            location !== "" &&
            travelingToLocation === "" &&
            location.substring(0, 5) === "wrld_"
          ) {
            debugger;
            // User HAS TRAVELED to instance
            let worldId = jsonData.worldId;
            let instanceId = location.split(":")[1];
            const resp = await getInstanceInfo(worldId, instanceId);
            let instanceData = await getInstanceInfo(worldId, instanceId).catch(
              (e) => {
                console.log(e.response);
                console.error("Error while trying to get instance data: " + e);
                return undefined;
              }
            );
            console.log(
              "User (" +
                jsonData.user.displayName +
                ") has traveled to world: [" +
                instanceData?.world.name +
                "]"
            );
            showToast(
              "User (" +
                jsonData.user.displayName +
                ") has traveled to world: [" +
                instanceData?.world.name +
                "]"
            );
            texts = [
              ...texts,
              "User (" +
                jsonData.user.displayName +
                ") has traveled to world: [" +
                instanceData?.world.name +
                "]",
            ];
          } else if (
            location === "private" &&
            jsonData.worldId === "private" &&
            travelingToLocation === "private"
          ) {
            console.log(
              "User (" +
                jsonData.user.displayName +
                ") is in a private instance (probably in a private world or in a private instance of a public world)"
            );
            showToast(
              "User (" +
                jsonData.user.displayName +
                ") is in a private instance (probably in a private world or in a private instance of a public world)"
            );
            texts = [
              ...texts,
              "User (" +
                jsonData.user.displayName +
                ") is in a private instance (probably in a private world or in a private instance of a public world)",
            ];
          } else {
            console.debug("PLEASE CHECK, NEEDS TO BE IMPLEMENTED");
            console.log(jsonData);
          }
          break;
        }

        default: {
          try {
            console.log(JSON.parse(event.data.toString()));
            console.log(
              await sha256(JSON.parse(event.data.toString()).content)
            );
          } catch (e) {
            console.error("Error while trying to parse message: " + e);
            console.log(JSON.parse(event.data.toString()));
          }
        }
      }
    };
    webSocket.onclose = function (event) {
      timerEnd = new Date();
      console.log("Disconnected from WebSocket server");
      showToast("Disconnected from WebSocket server");
      texts = [...texts, "Disconnected from WebSocket server"];
      wsTimer = Math.floor((timerEnd.getTime() - timerStart.getTime()) / 1000);
      texts = [...texts, "WebSocket was running for " + wsTimer + " seconds"];
      isWSRunning = false;
      if (event.reason === "MANUAL_CLOSE_BUTTON") {
        reRunWS();
      }
    };
    */
  }

  function doRunWS(authCookie: string) {
    if (!isWSRunning) {
      startWS(authCookie);
    }
  }

  function stopWS(event?: MouseEvent) {
    if (event) {
      webSocket.close(1000, "MANUAL_CLOSE_BUTTON");
    } else {
      webSocket.close();
    }
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
          await listen("ws_err", async (event: any) => {
            if (event.payload.err) {
              console.error("WebSocket VRChat ERROR: " + event.payload.err);
              texts = [
                ...texts,
                "WebSocket VRChat ERROR: " + event.payload.err,
              ];
            } else {
              console.error("WebSocket LOGIC error: " + event.payload);
              texts = [...texts, "WebSocket LOGIC error: " + event.payload];
            }
            invoke("stop_ws")
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
    invoke("stop_ws")
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
      // Open  a new window and navigate to the URL
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

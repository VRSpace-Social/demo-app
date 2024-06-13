<script lang="ts">
    let inputValue: string = '';
    async function sha256(message: string) {
        const msgBuffer = new TextEncoder().encode(message);
        const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
        const hashArray = Array.from(new Uint8Array(hashBuffer));
        const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
        return hashHex;
    }

    let texts: Array<string> = [];


    let wsTimer: number;
    let timerStart: Date;
    let timerEnd: Date;

    import { onMount, onDestroy } from "svelte";
    import { showToast } from '$utils/toast';
    import ReadOnlyDiv from '$lib/ReadOnlyDiv.svelte';
    import { goto } from '$app/navigation';

    let bkAuthCookie: string;
    let isWSRunning: boolean;
    let webSocket: WebSocket; // Declare webSocket as a global variable

    async function getInstanceInfo(worldId: string, instanceId: string) {
        debugger
        //TODO: Implement this function
        const result = await fetch('http://localhost:3000/api/getInstanceInfo?worldID=' + worldId + '&instanceID=' + instanceId);
        const jsonResult = await result.json();
        return jsonResult;
    }

    async function getUserInfo(userID: string) {
        const result = await fetch('http://localhost:3000/api/getUserInfo?userID=' + userID);
        return result.json();
    }

    function runWebSocket(authCookie: string) {
        webSocket = new WebSocket("wss://pipeline.vrchat.cloud/?authToken=" + authCookie);
        webSocket.onopen = function () {
            timerStart = new Date();
            console.log("Connected to WebSocket server");
            showToast("Connected to WebSocket server");
            texts = [...texts, "Connected to WebSocket server"];
            isWSRunning = true;
        };
        webSocket.onmessage = async function (event) {
            if (event.data === 'reload') {
                setTimeout(() => {
                    webSocket.close();
                    self.location.reload();
                }, 300);
            }

            if(JSON.parse(event.data.toString()).err) {
                texts = [...texts, JSON.parse(event.data.toString()).err];
            }
            
            switch (JSON.parse(event.data.toString()).type) {

                case "friend-active": {
                    let jsonData = JSON.parse(JSON.parse(event.data.toString()).content).user;
                    console.log(jsonData.displayName + " is now active (online on VRC Website or API)");
                    showToast(jsonData.displayName + " is now active (online on VRC Website or API)");
                    texts = [...texts, jsonData.displayName + " is now active (online on VRC Website or API)"];
                    break;
                }

                case "friend-offline": {
                    let friendData = await getUserInfo(JSON.parse(JSON.parse(event.data.toString()).content).userId).catch(e => {
                        let errorResponse = JSON.parse(e.response?.data);
                        console.log(errorResponse);
                        console.log("[!!] ERROR on" + JSON.parse(event.data.toString()) + "\nError while trying to get friend data: \n" + e);
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
                    console.debug(event.data.toString());
                    let friendData = await getUserInfo(JSON.parse(JSON.parse(event.data.toString()).content).userId).catch(e => {
                        console.log("[!!] ERROR on" + JSON.parse(event.data.toString()) + "\nError while trying to get friend data: \n" + e);
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
                    let friendUsername = JSON.parse(JSON.parse(event.data.toString()).content).user.displayName;
                    console.log(friendUsername + " is online on VRChat Client");
                    showToast(friendUsername + " is online on VRChat Client");
                    texts = [...texts, friendUsername + " is online on VRChat Client"];
                    break;
                }

                case "friend-location": {
                    let jsonData = JSON.parse(JSON.parse(event.data.toString()).content);
                    let location = jsonData.location;
                    let travelingToLocation = jsonData.travelingToLocation;
                    console.log(location)
                    console.log(travelingToLocation)
                    

                    if (location === "traveling" && travelingToLocation !== "" && jsonData.worldId.substring(0, 5) === "wrld_") {
                        // User IS TRAVELING to instance
                        debugger;
                        let worldId = jsonData.worldId;
                        let instanceId = jsonData.travelingToLocation.split(":")[1];
                        const resp = await getInstanceInfo(worldId, instanceId);
                        let instanceData = await getInstanceInfo(worldId, instanceId).catch(e => {
                            console.log(e.response)
                            console.error("Error while trying to get instance data: " + e);
                            return undefined;
                        });
                        console.log("User (" + jsonData.user.displayName + ") is traveling to world: [" + instanceData?.world.name + "]");
                        showToast("User (" + jsonData.user.displayName + ") is traveling to world: [" + instanceData?.world.name + "]");
                        texts = [...texts, "User (" + jsonData.user.displayName + ") is traveling to world: [" + instanceData?.world.name + "]"];

                    } else if (location !== "" && travelingToLocation === "" && location.substring(0, 5) === "wrld_") {
                        debugger;
                        // User HAS TRAVELED to instance
                        let worldId = jsonData.worldId;
                        let instanceId = location.split(":")[1];
                        const resp = await getInstanceInfo(worldId, instanceId);
                        let instanceData = await getInstanceInfo(worldId, instanceId).catch(e => {
                            console.log(e.response)
                            console.error("Error while trying to get instance data: " + e);
                            return undefined;
                        });
                        console.log("User (" + jsonData.user.displayName + ") has traveled to world: [" + instanceData?.world.name + "]");
                        showToast("User (" + jsonData.user.displayName + ") has traveled to world: [" + instanceData?.world.name + "]");
                        texts = [...texts, "User (" + jsonData.user.displayName + ") has traveled to world: [" + instanceData?.world.name + "]"];
                    }
                    else if (location === "private" && jsonData.worldId === "private" && travelingToLocation === "private") {
                        console.log("User (" + jsonData.user.displayName + ") is in a private instance (probably in a private world or in a private instance of a public world)");
                        showToast("User (" + jsonData.user.displayName + ") is in a private instance (probably in a private world or in a private instance of a public world)");
                        texts = [...texts, "User (" + jsonData.user.displayName + ") is in a private instance (probably in a private world or in a private instance of a public world)"];
                    }
                    else {
                        console.debug("PLEASE CHECK, NEEDS TO BE IMPLEMENTED");
                        console.log(jsonData);
                    }
                    break;
                }

                default: {
                    try {
                        console.log(JSON.parse(event.data.toString()));
                        console.log(await sha256(JSON.parse(event.data.toString()).content));
                    } catch (e) {
                        console.error("Error while trying to parse message: " + e)
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
            wsTimer = // In seconds
                Math.floor((timerEnd.getTime() - timerStart.getTime()) / 1000);
            texts = [...texts, "WebSocket was running for " + wsTimer + " seconds"];
            isWSRunning = false;
        };
    }

    function doRunWS(authCookie: string) {
        if (!isWSRunning) {
            runWebSocket(authCookie);
        }
    }

    function stopWS() {
        if (webSocket) {
            webSocket.close();
        }
    }

    onMount(async () => {
        console.log("Welcome to VRChat WebSocket using VRSpace API!");
        const authCookieRequest = await fetch("http://localhost:3000/api/getAuthCookie", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            }
        }).catch(e => {
            console.error("Error while trying to get auth cookie: " + e);
            return undefined;
        });
        const authCookie = await authCookieRequest?.text();
        if(authCookie) {
            bkAuthCookie = authCookie;
            doRunWS(authCookie);
        } else {
            console.error("Auth cookie is not found");
            showToast('Auth cookie is not found');
            texts = [...texts, 'Auth cookie is not found'];

        }
    });

    onDestroy(() => {
        stopWS();
    });

    function handleSubmit() {
        if (inputValue) {
            // Open  a new window and navigate to the URL
            console.log(inputValue)
            goto(`/about/${inputValue}`);
        }
    }

    function reRunWS() {
        if(!isWSRunning) {
            doRunWS(bkAuthCookie);
        }
    }

</script>

<h1>VRChat WebSocket using VRSpace API</h1>

<button on:click={stopWS}>Stop WS</button>

<button on:click={reRunWS}>Restart WS</button>


<form action="" on:submit|preventDefault={handleSubmit}>
    <input type="text" bind:value={inputValue}>
</form>

<ReadOnlyDiv {texts} />

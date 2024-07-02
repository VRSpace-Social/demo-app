<script lang="ts">
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import { showToast } from '$utils/toast';
  import { readStore } from '$utils/store';
  import { Card } from 'svelte-ux';
  import { Button } from 'svelte-ux';

  interface FriendOnlineData {
    username: string,
    worldName?: string,
    worldId?: string
    instanceId?: string
    instanceType: string
    players?: number
    maxPlayers?: number
    worldImageUrl?: string | null | undefined,
    userImageUrl: string | undefined,
    canJoin: boolean
  }

  interface FriendSearchData {
    username: string,
    worldName?: string,
    worldId?: string
    instanceId?: string
    instanceType?: string
    players?: number
    maxPlayers?: number
    worldImageUrl?: string | null | undefined,
    userImageUrl: string | undefined,
    canJoin?: boolean
  }

  const storeData = readStore();
  let isVRSpaceRunning: boolean = !!storeData;
  const onlineFriends = writable<Array<FriendOnlineData>>([]);
  const privateFriends = writable<Array<FriendOnlineData>>([]);
  const websiteFriends = writable<Array<FriendOnlineData>>([]);
  const isLoading = writable(true);

  const fetchFriendsData = async () => {
    const response = await fetch("http://localhost:3000/api/getOnlineFriends");
    return await response.json();
  };

  const displayFriends = (friendsData: Array<FriendOnlineData>) => {
    friendsData.forEach((friend) => {
      if (friend.instanceType === "Private Instance") {
        privateFriends.update((friends) => [...friends, friend]);
      } else if (friend.instanceType === "Is online on VRChat Website/API") {
        websiteFriends.update((friends) => [...friends, friend]);
      } else {
        onlineFriends.update((friends) => [...friends, friend]);
      }
    });
  };

  onMount(async () => {
    if(isVRSpaceRunning) {
      showToast("VRSpace server is running!");
    }
    const friendsData = await fetchFriendsData();
    displayFriends(friendsData);
    isLoading.set(false);    
  });
</script>

<main>
    <div class="container mt-4">
      {#if $isLoading}
        <div class="spinner-container">
          <div class="spinner-border text-light" role="status">
            <span class="sr-only">Loading...</span>
          </div>
          <div>Loading friends data, please wait...</div>
        </div>
      {:else}
        <div>
          <h2 class="category-title">Friends in Worlds you can join!</h2>
          <div class="grid grid-cols-3 gap-3">
            {#each $onlineFriends as friend}
              <Card title="{friend.username} is playing" subheading="{friend.worldName}">
                <img src={friend.worldImageUrl || "default-image.jpg"} alt={friend.worldName}  />
                <div slot="actions">
                  <a href={`vrchat://launch?ref=vrchat.com&id=${friend.instanceId}`}>
                    <Button>Join</Button>
                  </a>
                  <Button disabled>{friend.instanceType}</Button>
                </div>
                <p>
                  <small>Players: {friend.players}/{friend.maxPlayers}</small>
                </p>
              </Card>
            {/each}
          </div>
        </div>

        <div>
          <h2 class="category-title">Friends in Private Worlds</h2>
          <div class="grid grid-cols-3 gap-3">
            {#each $privateFriends as friend}
              <Card title="{friend.username} is playing" subheading="{friend.worldName}">
                <img src={friend.userImageUrl || "default-image.jpg"} alt={friend.worldName} />
                <div slot="actions">
                  <Button disabled>{friend.instanceType}</Button>
                </div>
                <p>
                  <small>Players: {friend.players}/{friend.maxPlayers}</small>
                </p>
              </Card>
            {/each}
          </div>
        </div>

        <div>
          <h2 class="category-title">Friends Active on the Website</h2>
          <div class="grid grid-cols-3 gap-3">
            {#each $websiteFriends as friend}
              <Card title="{friend.username} is playing" subheading="{friend.worldName}">
                <img src={friend.userImageUrl || "default-image.jpg"} alt={friend.worldName} />
                <div slot="actions">
                  <Button disabled>{friend.instanceType}</Button>
                </div>
                <p >
                  <small>Players: {friend.players}/{friend.maxPlayers}</small>
                </p>
              </Card>
            {/each}
          </div>
        </div>
      {/if}
    </div>
</main>

<style>
  body {
    background-color: #2e2e2e;
    color: white;
  }

  .grid {
    display: grid;
  }

  .grid-cols-3 {
    grid-template-columns: repeat(3, 1fr);
  }

  .gap-3 {
    gap: 20px;
  }

  .category-title {
    margin-top: 20px;
    margin-bottom: 10px;
    text-align: center;
  }

  .spinner-container {
    text-align: center;
    margin-top: 20px;
  }
</style>

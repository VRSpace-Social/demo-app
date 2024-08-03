<script lang="ts">
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
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import { showToast } from '$utils/toast';
  import { readStore } from '$utils/store';

  const storeData = readStore();
  let isVRSpaceRunning: boolean = storeData ? true : false;
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
  <head>
    <link
      rel="stylesheet"
      href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
    />
  </head>
  <body>
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
          <div class="row">
            {#each $onlineFriends as friend}
              <div class="col-md-4">
                <div class="card friend-card">
                  <img
                    src={friend.worldImageUrl || friend.userImageUrl}
                    class="card-img-top"
                    alt={friend.worldName}
                  />
                  <div class="card-body">
                    <h5 class="card-title">{friend.username} is playing</h5>
                    {#if friend.canJoin}
                      <a
                        href={`vrchat://launch?ref=vrchat.com&id=${friend.instanceId}`}
                        ><i class="fa-solid fa-earth-americas"></i>
                        {friend.worldName}</a
                      >
                    {:else}
                      <i class="fa-solid fa-earth-americas"></i>
                      {friend.worldName}
                    {/if}
                    <p class="card-text">
                      <small
                        >Players: {friend.players}/{friend.maxPlayers}</small
                      >
                      <br />
                      <small>{friend.instanceType}</small>
                    </p>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <div>
          <h2 class="category-title">Friends in Private Worlds</h2>
          <div class="row">
            {#each $privateFriends as friend}
              <div class="col-md-4">
                <div class="card friend-card">
                  <img
                    src={friend.worldImageUrl || friend.userImageUrl}
                    class="card-img-top"
                    alt={friend.worldName}
                  />
                  <div class="card-body">
                    <h5 class="card-title">{friend.username} is playing</h5>
                    <i class="fa-solid fa-earth-americas"></i>
                    {friend.worldName}
                    <p class="card-text">
                      <small
                        >Players: {friend.players}/{friend.maxPlayers}</small
                      >
                      <br />
                      <small>{friend.instanceType}</small>
                    </p>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <div>
          <h2 class="category-title">Friends Active on the Website</h2>
          <div class="row">
            {#each $websiteFriends as friend}
              <div class="col-md-4">
                <div class="card friend-card">
                  <img
                    src={friend.worldImageUrl || friend.userImageUrl}
                    class="card-img-top"
                    alt={friend.worldName}
                  />
                  <div class="card-body">
                    <h5 class="card-title">{friend.username} is playing</h5>
                    <i class="fa-solid fa-earth-americas"></i>
                    {friend.worldName}
                    <p class="card-text">
                      <small
                        >Players: {friend.players}/{friend.maxPlayers}</small
                      >
                      <br />
                      <small>{friend.instanceType}</small>
                    </p>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </body>
</main>

<style>
  body {
    background-color: #2e2e2e;
    color: white;
  }

  .friend-card {
    background-color: #3b3b3b;
    border-radius: 10px;
    overflow: hidden;
    margin-bottom: 10px;
  }

  .friend-card img {
    height: 261px;
    object-fit: cover;
  }

  .friend-card .card-body {
    padding: 10px;
  }

  .friend-card:hover {
    background-color: #4b4b4b;
  }

  .category-title {
    margin-top: 20px;
    margin-bottom: 10px;
  }

  .spinner-container {
    text-align: center;
    margin-top: 20px;
  }
</style>

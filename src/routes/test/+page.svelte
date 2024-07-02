<script lang="ts">

  import VrcUserCard from "$lib/vrcUserCard.svelte";

</script>


<main>
    <!-- Div Container che fa vedere due componenti Svelte sulla stessa riga al centro con un margin -->
    <div style="display: flex; justify-content: center; margin: 20px;">
        <VrcUserCard 
            username="Test User" 
            profilePicture="https://api.vrchat.cloud/api/1/image/file_e3bf624c-8dc9-4e38-a4bb-1e40e8fb6e3a/2/1024" 
            worldPicture="https://api.vrchat.cloud/api/1/image/file_635cd89e-9fe4-4c6d-93b4-8ccd7413d350/7/256" 
            instanceType="Public" 
            playerCounter="1" 
            maxPlayerCount="16" 
            canJoin={true} 
            instanceId="1234567890" 
            worldName="Test World"
            vrcInstanceUrl="https://vrchat.com/home"
        />

        <VrcUserCard 
            username="Test User" 
            profilePicture="https://api.vrchat.cloud/api/1/image/file_e3bf624c-8dc9-4e38-a4bb-1e40e8fb6e3a/2/1024" 
            worldPicture="https://api.vrchat.cloud/api/1/image/file_29046a79-a16f-4e87-a393-ff6be2d8dd23/17/1024" 
            instanceType="Private Instance" 
            playerCounter="1" 
            maxPlayerCount="1" 
            canJoin={false} 
            instanceId="1234567890" 
            worldName="Just B Club"
            vrcInstanceUrl="https://vrchat.com/home"
        />
    </div>
</main>

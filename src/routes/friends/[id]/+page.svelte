<script>
	/** @type {import('./$types').PageData} */
	export let data;
</script>


<style>
    .card {
        display: flex;
        flex-direction: column;
        max-width: 300px;
        background-color: #2a2a2a;
        color: white;
        border-radius: 10px;
        overflow: hidden;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    }
    .card img {
        width: 100%;
        height: auto;
        border-bottom: 2px solid #444;
    }
    .card-content {
        padding: 15px;
    }
    .card-title {
        font-size: 1.2em;
        margin-bottom: 10px;
    }
    .card-details {
        font-size: 0.9em;
        color: #bbb;
    }
    .icon {
        width: 20px;
        height: 20px;
        margin-right: 5px;
    }
</style>

<div class="card">
    <img src="{data.post?.picture}" alt="{data.post?.title}">
    <div class="card-content">
        <div class="card-title">{data.post?.title} is playing</div>
        <div class="card-details">
            <div>
                🌎 Players: 0/0
            </div>
            <div>Is online on VRChat Website/API</div>
        </div>
    </div>
</div>

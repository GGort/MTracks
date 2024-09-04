<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {Button} from "flowbite-svelte";

</script>

<h1>Home</h1>

<Button href="/songs">Songs</Button>

<Button on:click={()=>invoke('get_songs')}>test</Button>
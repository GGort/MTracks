<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {Button} from "flowbite-svelte";

</script>

<h1>Home</h1>
<Button on:click={()=>invoke('play_test_sound')}>play_test</Button>